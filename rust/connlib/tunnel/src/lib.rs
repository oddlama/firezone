//! Connlib tunnel implementation.
//!
//! This is both the wireguard and ICE implementation that should work in tandem.
//! [Tunnel] is the main entry-point for this crate.
use boringtun::x25519::StaticSecret;

use connlib_shared::{messages::ReuseConnection, CallbackErrorFacade, Callbacks, Error};
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use if_watch::tokio::IfWatcher;
use ip_network_table::IpNetworkTable;
use pnet_packet::Packet;
use snownet::{Client, Node, Server, IpPacket};

use hickory_resolver::proto::rr::RecordType;
use parking_lot::Mutex;
use peer::{PacketTransform, Peer};
use sockets::{Socket, UdpSockets};
use tokio::time::MissedTickBehavior;

use arc_swap::ArcSwapOption;
use futures_util::task::AtomicWaker;
use std::{collections::HashSet, hash::Hash};
use std::{
    collections::VecDeque,
    net::{Ipv4Addr, Ipv6Addr},
};
use std::{fmt, net::IpAddr, sync::Arc, time::Duration};
use std::{
    task::{ready, Context, Poll},
    time::Instant,
};
use tokio::time::Interval;

use connlib_shared::{
    messages::{GatewayId, ResourceDescription},
    Result,
};

pub use client::ClientState;
use connlib_shared::error::ConnlibError;
pub use control_protocol::Request;
pub use gateway::GatewayState;

use crate::ip_packet::MutableIpPacket;
use connlib_shared::messages::ClientId;
use device_channel::Device;

mod bounded_queue;
mod client;
mod control_protocol;
mod device_channel;
mod dns;
mod gateway;
mod ip_packet;
mod peer;
mod sockets;

const MAX_UDP_SIZE: usize = (1 << 16) - 1;
const DNS_QUERIES_QUEUE_SIZE: usize = 100;

const REALM: &str = "firezone";

pub(crate) fn get_v4(ip: IpAddr) -> Option<Ipv4Addr> {
    match ip {
        IpAddr::V4(v4) => Some(v4),
        IpAddr::V6(_) => None,
    }
}

pub(crate) fn get_v6(ip: IpAddr) -> Option<Ipv6Addr> {
    match ip {
        IpAddr::V4(_) => None,
        IpAddr::V6(v6) => Some(v6),
    }
}

struct Connections<TRole, TId> {
    pub connection_pool: Node<TRole, TId>,
    connection_pool_timeout: BoxFuture<'static, std::time::Instant>,
    if_watcher: IfWatcher,
    udp_sockets: UdpSockets<MAX_UDP_SIZE>,
    relay_socket: Socket<MAX_UDP_SIZE>,
}

impl<TRole, TId> Connections<TRole, TId>
where
    TId: Eq + Hash + Copy + fmt::Display,
{
    fn new(private_key: StaticSecret) -> Self {
        let if_watcher = IfWatcher::new().expect(
            "Program should be able to list interfaces on the system. Check binary's permissions",
        );

        let mut connection_pool = Node::new(private_key, std::time::Instant::now());
        let mut udp_sockets = UdpSockets::default();

        for ip in if_watcher.iter() {
            tracing::info!(address = %ip.addr(), "New local interface address found");
            match udp_sockets.bind((ip.addr(), 0)) {
                Ok(addr) => {
                    tracing::debug!(%addr, "Adding address to connection pool");
                    connection_pool.add_local_interface(addr)
                }
                Err(e) => {
                    tracing::debug!(address = %ip.addr(), err = ?e, "Couldn't bind socket to interface: {e:#?}")
                }
            }
        }

        let relay_socket = Socket::bind((IpAddr::from(Ipv4Addr::UNSPECIFIED), 0))
            .expect("Program should be able to bind to 0.0.0.0:0 to be able to connect to relays");
        // TODO: right now the connection pool expects a socket address that it has already seen
        // this will be relaxed on a later PR.
        connection_pool.add_local_interface(relay_socket.local_addr());

        Connections {
            connection_pool,
            connection_pool_timeout: sleep_until(std::time::Instant::now()).boxed(),
            if_watcher,
            udp_sockets,
            relay_socket,
        }
    }

    fn send(&mut self, id: TId, packet: IpPacket) -> Result<()> {
        let Some(transmit) = self.connection_pool.encapsulate(id, packet)? else {
            return Ok(());
        };

        match transmit.src {
            Some(src) => self
                .udp_sockets
                .try_send_to(src, transmit.dst, &transmit.payload)?,
            None => self
                .relay_socket
                .try_send_to(transmit.dst, &transmit.payload)?,
        };

        tracing::trace!(target: "wire", action = "write", to = %transmit.dst, src = ?transmit.src, bytes = %transmit.payload.len());

        Ok(())
    }

    fn poll_sockets<'a>(
        &mut self,
        write_buf: &'a mut [u8],
        cx: &mut Context<'_>,
    ) -> Poll<Option<firezone_connection::IpPacket<'a>>> {
        match self.udp_sockets.poll_recv_from(cx) {
            Poll::Ready((local, Ok((from, packet)))) => {
                match self.connection_pool.decapsulate(
                    local,
                    from,
                    packet.filled(),
                    std::time::Instant::now(),
                    write_buf,
                ) {
                    Ok(Some((conn_id, packet))) => {
                        tracing::trace!(target: "wire", %local, %from, bytes = %packet.packet().len(), "read new packet");
                        return Poll::Ready(Some(packet));
                    }
                    Ok(None) => return Poll::Ready(None),
                    Err(e) => {
                        tracing::error!(%local, %from, "Failed to decapsulate incoming packet: {e:#?}");
                        return Poll::Ready(None);
                    }
                }
            }
            Poll::Ready((addr, Err(e))) => {
                tracing::error!(%addr, "Failed to read socket: {e:#?}");
                return Poll::Ready(None);
            }
            Poll::Pending => {}
        }

        match ready!(self.relay_socket.poll_recv_from(cx)) {
            (local, Ok((from, packet))) => {
                match self.connection_pool.decapsulate(
                    local,
                    from,
                    packet.filled(),
                    std::time::Instant::now(),
                    write_buf,
                ) {
                    Ok(Some((conn_id, packet))) => {
                        tracing::trace!(target: "wire", %from, bytes = %packet.packet().len(), "read new relay packet");
                        return Poll::Ready(Some(packet));
                    }
                    Ok(None) => {
                        return Poll::Ready(None);
                    }
                    Err(e) => {
                        tracing::error!(%from, "Failed to decapsulate incoming relay packet: {e:#?}");
                        return Poll::Ready(None);
                    }
                }
            }
            (_, Err(e)) => {
                tracing::error!("Failed to read relay socket: {e:#?}");
                return Poll::Ready(None);
            }
        }
    }

    fn poll_next_event(&mut self, cx: &mut Context<'_>) -> Poll<Event<TId>> {
        loop {
            while let Some(transmit) = self.connection_pool.poll_transmit() {
                if let Err(e) = match transmit.src {
                    Some(src) => self
                        .udp_sockets
                        .try_send_to(src, transmit.dst, &transmit.payload),
                    None => self
                        .relay_socket
                        .try_send_to(transmit.dst, &transmit.payload),
                } {
                    tracing::warn!(src = ?transmit.src, dst = %transmit.dst, "Failed to send UDP packet: {e:#?}");
                }
            }

            match self.connection_pool.poll_event() {
                Some(snownet::Event::SignalIceCandidate {
                    connection,
                    candidate,
                }) => {
                    tracing::debug!(%candidate, %connection, "New local candidate");
                    return Poll::Ready(Event::SignalIceCandidate {
                        conn_id: connection,
                        candidate,
                    });
                }
                Some(snownet::Event::ConnectionEstablished(id)) => {
                    tracing::info!(gateway_id = %id, "Connection established with peer");
                    // TODO (We probably don't need to do anything here)
                }
                Some(snownet::Event::ConnectionFailed(id)) => {
                    tracing::info!(gateway_id = %id, "Connection failed with peer");
                    // TODO (We need to cleanup the peer since we create it before we get establish the connection)
                }
                None => {}
            }

            if let Poll::Ready(instant) = self.connection_pool_timeout.poll_unpin(cx) {
                self.connection_pool.handle_timeout(instant);
                if let Some(timeout) = self.connection_pool.poll_timeout() {
                    self.connection_pool_timeout = sleep_until(timeout).boxed();
                }

                continue;
            }

            match self.if_watcher.poll_if_event(cx) {
                Poll::Ready(Ok(ev)) => match ev {
                    if_watch::IfEvent::Up(ip) => {
                        tracing::info!(address = %ip.addr(), "New local interface address found");
                        match self.udp_sockets.bind((ip.addr(), 0)) {
                            Ok(addr) => {
                                tracing::debug!(%addr, "Adding address to connection pool");
                                self.connection_pool.add_local_interface(addr)
                            }
                            Err(e) => {
                                tracing::debug!(address = %ip.addr(), err = ?e, "Couldn't bind socket to interface: {e:#?}")
                            }
                        }
                    }
                    if_watch::IfEvent::Down(ip) => {
                        tracing::info!(address = %ip.addr(), "Interface IP no longer available");
                        todo!()
                    }
                },
                Poll::Ready(Err(e)) => {
                    tracing::debug!("Error while polling interfces: {e:#?}");
                }
                Poll::Pending => {}
            }

            return Poll::Pending;
        }
    }
}

pub type GatewayTunnel<CB> = Tunnel<CB, GatewayState, Server, ClientId>;
pub type ClientTunnel<CB> = Tunnel<CB, ClientState, Client, GatewayId>;

/// Tunnel is a wireguard state machine that uses webrtc's ICE channels instead of UDP sockets to communicate between peers.
pub struct Tunnel<CB: Callbacks, TRoleState: RoleState, TRole, TId> {
    // TODO: these are used to stop connections
    // peer_connections: Mutex<HashMap<TRoleState::Id, Arc<RTCIceTransport>>>,
    callbacks: CallbackErrorFacade<CB>,

    /// State that differs per role, i.e. clients vs gateways.
    role_state: Mutex<TRoleState>,

    peer_refresh_interval: Mutex<Interval>,
    mtu_refresh_interval: Mutex<Interval>,

    peers_to_stop: Mutex<VecDeque<TRoleState::Id>>,

    device: Arc<ArcSwapOption<Device>>,
    read_buf: Mutex<Box<[u8; MAX_UDP_SIZE]>>,
    no_device_waker: AtomicWaker,

    connections: Mutex<Connections<TRole, TId>>,
    write_buf: Mutex<Box<[u8; MAX_UDP_SIZE]>>,
}

impl<CB> Tunnel<CB, ClientState, Client, GatewayId>
where
    CB: Callbacks + 'static,
{
    pub async fn next_event(&self) -> Result<Event<GatewayId>> {
        std::future::poll_fn(|cx| loop {
            {
                let guard = self.device.load();

                if let Some(device) = guard.as_ref() {
                    match self.poll_device(device, cx) {
                        Poll::Ready(Ok(Some(event))) => return Poll::Ready(Ok(event)),
                        Poll::Ready(Ok(None)) => {
                            tracing::info!("Device stopped");
                            self.device.store(None);
                            continue;
                        }
                        Poll::Ready(Err(e)) => {
                            self.device.store(None); // Ensure we don't poll a failed device again.
                            return Poll::Ready(Err(e));
                        }
                        Poll::Pending => {}
                    }
                } else {
                    self.no_device_waker.register(cx.waker());
                }
            }

            match self.poll_next_event_common(cx) {
                Poll::Ready(event) => return Poll::Ready(Ok(event)),
                Poll::Pending => {}
            }

            // TODO: we can get better about these multiple locks
            // if we move device and peer_by_ip to connections this problem goes away
            match ready!(self
                .connections
                .lock()
                .poll_sockets(self.write_buf.lock().as_mut(), cx))
            {
                Some(p) => {
                    if let Some(peer) = peer_by_ip(&self.role_state.lock().peers_by_ip, p.source())
                    {
                        match peer.untransform(p.source(), self.write_buf.lock().as_mut()) {
                            Ok(p) => {
                                let dev = self.device.load();
                                let Some(dev) = dev.as_ref() else {
                                    continue;
                                };

                                if let Err(e) = dev.write(p) {
                                    tracing::error!("Error writing packet to device: {e:?}");
                                }
                            }
                            Err(e) => {
                                tracing::error!(from = %p.source(), to = %p.destination(), "Error handling packet: {e:#?}");
                            }
                        }
                    }
                }

                None => {}
            }

        })
        .await
    }

    pub(crate) fn poll_device(
        &self,
        device: &Device,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<Event<GatewayId>>>> {
        loop {
            let mut role_state = self.role_state.lock();

            let mut read_guard = self.read_buf.lock();
            let read_buf = read_guard.as_mut_slice();

            let Some(packet) = ready!(device.poll_read(read_buf, cx))? else {
                return Poll::Ready(Ok(None));
            };

            tracing::trace!(target: "wire", action = "read", from = "device", dest = %packet.destination());

            let (packet, dest) = match role_state.handle_dns(packet) {
                Ok(Some(response)) => {
                    device.write(response)?;
                    continue;
                }
                Ok(None) => continue,
                Err(non_dns_packet) => non_dns_packet,
            };

            let Some(peer) = peer_by_ip(&role_state.peers_by_ip, dest) else {
                role_state.on_connection_intent_ip(dest);
                continue;
            };

            self.send_peer(packet, peer);

            continue;
        }
    }
}

impl<CB> Tunnel<CB, GatewayState, Server, ClientId>
where
    CB: Callbacks + 'static,
{
    pub fn poll_next_event(&self, cx: &mut Context<'_>) -> Poll<Result<Event<ClientId>>> {
        let mut read_guard = self.read_buf.lock();
        let read_buf = read_guard.as_mut_slice();

        loop {
            {
                let device = self.device.load();

                match device.as_ref().map(|d| d.poll_read(read_buf, cx)) {
                    Some(Poll::Ready(Ok(Some(packet)))) => {
                        let dest = packet.destination();

                        let role_state = self.role_state.lock();
                        let Some(peer) = peer_by_ip(&role_state.peers_by_ip, dest) else {
                            continue;
                        };

                        self.send_peer(packet, peer);

                        continue;
                    }
                    Some(Poll::Ready(Ok(None))) => {
                        tracing::info!("Device stopped");
                        self.device.store(None);
                    }
                    Some(Poll::Ready(Err(e))) => return Poll::Ready(Err(ConnlibError::Io(e))),
                    Some(Poll::Pending) => {
                        // device not ready for reading, moving on ..
                    }
                    None => {
                        self.no_device_waker.register(cx.waker());
                    }
                }
            }

            match self.poll_next_event_common(cx) {
                Poll::Ready(e) => return Poll::Ready(Ok(e)),
                Poll::Pending => {}
            }

            // TODO: we can get better about these multiple locks
            match ready!(self
                .connections
                .lock()
                .poll_sockets(self.write_buf.lock().as_mut(), cx))
            {
                Some(p) => {
                    if let Some(peer) = peer_by_ip(&self.role_state.lock().peers_by_ip, p.source())
                    {
                        match peer.untransform(p.source(), self.write_buf.lock().as_mut()) {
                            Ok(p) => {
                                let dev = self.device.load();
                                let Some(dev) = dev.as_ref() else {
                                    continue;
                                };

                                if let Err(e) = dev.write(p) {
                                    tracing::error!("Error writing packet to device: {e:?}");
                                }
                            }
                            Err(e) => {
                                tracing::error!(from = %p.source(), to = %p.destination(), "Error handling packet: {e:#?}");
                            }
                        }
                    }
                }

                None => {}
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TunnelStats {
    // public_key: String,
    // TODO:
    // peer_connections: Vec<TId>,
}

impl<CB, TRoleState, TRole, TId> Tunnel<CB, TRoleState, TRole, TId>
where
    CB: Callbacks + 'static,
    TRoleState: RoleState<Id = TId>,
    TId: Eq + Hash + Copy + fmt::Display,
{
    pub fn stats(&self) -> TunnelStats {
        // TODO:
        // let peer_connections = self.peer_connections.lock().keys().cloned().collect();

        TunnelStats {
            // public_key: Key::from(self.public_key).to_string(),
            // TODO:
            // peer_connections,
        }
    }

    fn poll_next_event_common(&self, cx: &mut Context<'_>) -> Poll<Event<TRoleState::Id>> {
        loop {
            if let Some(conn_id) = self.peers_to_stop.lock().pop_front() {
                self.role_state.lock().remove_peers(conn_id);

                // TODO: Stop the connection
                // if let Some(conn) = self.peer_connections.lock().remove(&conn_id) {
                //     tokio::spawn({
                //         async move {
                //             if let Err(e) = conn.stop().await {
                //                 tracing::warn!(%conn_id, error = ?e, "Can't close peer");
                //             }
                //         }
                //     });
                // }
            }

            if self.peer_refresh_interval.lock().poll_tick(cx).is_ready() {
                let mut peers_to_stop = self.role_state.lock().refresh_peers();
                self.peers_to_stop.lock().append(&mut peers_to_stop);

                continue;
            }

            if self.mtu_refresh_interval.lock().poll_tick(cx).is_ready() {
                let Some(device) = self.device.load().clone() else {
                    tracing::debug!("Device temporarily not available");
                    continue;
                };

                if let Err(e) = device.refresh_mtu() {
                    tracing::error!(error = ?e, "refresh_mtu");
                }
            }

            // TODO: we are holding 2 mutexes here. Fix this.
            if let Poll::Ready(event) = self.connections.lock().poll_next_event(cx) {
                return Poll::Ready(event);
            }

            if let Poll::Ready(event) = self.role_state.lock().poll_next_event(cx) {
                return Poll::Ready(event);
            }

            return Poll::Pending;
        }
    }

    fn send_peer<TTransform: PacketTransform>(
        &self,
        packet: MutableIpPacket,
        peer: &Peer<TRoleState::Id, TTransform>,
    ) {
        let Some(p) = peer.transform(packet) else {
            return;
        };

        if let Err(e) = self
            .connections
            .lock()
            .send(peer.conn_id, p.as_immutable().into())
        {
            tracing::error!(to = %p.destination(), conn_id = %peer.conn_id, "Failed to send packet: {e:#?}");
        }
    }
}

pub(crate) fn peer_by_ip<Id, TTransform>(
    peers_by_ip: &IpNetworkTable<Arc<Peer<Id, TTransform>>>,
    ip: IpAddr,
) -> Option<&Peer<Id, TTransform>> {
    peers_by_ip.longest_match(ip).map(|(_, peer)| peer.as_ref())
}

#[derive(Debug)]
pub struct DnsQuery<'a> {
    pub name: String,
    pub record_type: RecordType,
    // We could be much more efficient with this field,
    // we only need the header to create the response.
    pub query: crate::ip_packet::IpPacket<'a>,
}

impl<'a> DnsQuery<'a> {
    pub(crate) fn into_owned(self) -> DnsQuery<'static> {
        let Self {
            name,
            record_type,
            query,
        } = self;
        let buf = query.packet().to_vec();
        let query = ip_packet::IpPacket::owned(buf)
            .expect("We are constructing the ip packet from an ip packet");

        DnsQuery {
            name,
            record_type,
            query,
        }
    }
}

pub enum Event<TId> {
    SignalIceCandidate {
        conn_id: TId,
        candidate: String,
    },
    ConnectionIntent {
        resource: ResourceDescription,
        connected_gateway_ids: HashSet<GatewayId>,
        reference: usize,
    },
    RefreshResources {
        connections: Vec<ReuseConnection>,
    },
    DnsQuery(DnsQuery<'static>),
}

impl<CB, TRoleState, TRole, TId> Tunnel<CB, TRoleState, TRole, TId>
where
    CB: Callbacks + 'static,
    TRoleState: RoleState<Id = TId>,
    TId: Eq + Hash + Copy + fmt::Display,
{
    /// Creates a new tunnel.
    ///
    /// # Parameters
    /// - `private_key`: wireguard's private key.
    /// -  `control_signaler`: this is used to send SDP from the tunnel to the control plane.
    #[tracing::instrument(level = "trace", skip(private_key, callbacks))]
    pub async fn new(private_key: StaticSecret, callbacks: CB) -> Result<Self> {
        // TODO:
        // let peer_connections = Default::default();

        // TODO:
        // setting_engine.set_interface_filter(Box::new(|name| !name.contains("tun")));

        Ok(Self {
            // TODO:
            // peer_connections,
            device: Default::default(),
            read_buf: Mutex::new(Box::new([0u8; MAX_UDP_SIZE])),
            callbacks: CallbackErrorFacade(callbacks),
            role_state: Default::default(),
            peer_refresh_interval: Mutex::new(peer_refresh_interval()),
            mtu_refresh_interval: Mutex::new(mtu_refresh_interval()),
            peers_to_stop: Default::default(),
            no_device_waker: Default::default(),
            connections: Mutex::new(Connections::new(private_key)),
            write_buf: Mutex::new(Box::new([0; MAX_UDP_SIZE])),
        })
    }

    pub fn callbacks(&self) -> &CallbackErrorFacade<CB> {
        &self.callbacks
    }
}

/// Constructs the interval for "refreshing" peers.
///
/// On each tick, we remove expired peers from our map, update wireguard timers and send packets, if any.
fn peer_refresh_interval() -> Interval {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    interval
}

/// Constructs the interval for refreshing the MTU of our TUN device.
fn mtu_refresh_interval() -> Interval {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    interval
}

/// Dedicated trait for abstracting over the different ICE states.
///
/// By design, this trait does not allow any operations apart from advancing via [`RoleState::poll_next_event`].
/// The state should only be modified when the concrete type is known, e.g. [`ClientState`] or [`GatewayState`].
pub trait RoleState: Default + Send + 'static {
    type Id: fmt::Debug + fmt::Display + Eq + Hash + Copy + Unpin + Send + Sync + 'static;

    fn poll_next_event(&mut self, cx: &mut Context<'_>) -> Poll<Event<Self::Id>>;
    fn remove_peers(&mut self, conn_id: Self::Id);
    fn refresh_peers(&mut self) -> VecDeque<Self::Id>;
}

async fn sleep_until(deadline: Instant) -> Instant {
    tokio::time::sleep_until(deadline.into()).await;

    deadline
}
