use ip_packet::IpPacket;
use rangemap::RangeInclusiveSet;

use crate::messages::gateway::{Filter, Filters};

#[derive(Debug)]
pub(crate) enum FilterEngine {
    PermitAll,
    PermitSome(AllowRules),
}

#[derive(Debug)]
pub(crate) struct AllowRules {
    udp: RangeInclusiveSet<u16>,
    tcp: RangeInclusiveSet<u16>,
    icmp: bool,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum Filtered {
    #[error("TCP port is not in allowed range")]
    Tcp,
    #[error("UDP port is not in allowed range")]
    Udp,
    #[error("ICMP not allowed")]
    Icmp,
}

impl FilterEngine {
    pub(crate) fn apply(&self, packet: &IpPacket) -> Result<(), Filtered> {
        match self {
            FilterEngine::PermitAll => Ok(()),
            FilterEngine::PermitSome(filter_engine) => filter_engine.apply(packet),
        }
    }

    pub(crate) fn with_filters<'a>(
        filters: impl Iterator<Item = &'a Filters> + Clone,
    ) -> FilterEngine {
        // Empty filters means permit all
        if filters.clone().any(|f| f.is_empty()) {
            return Self::PermitAll;
        }

        let mut allow_rules = AllowRules::new();
        allow_rules.add_filters(filters.flatten());

        Self::PermitSome(allow_rules)
    }
}

impl AllowRules {
    fn new() -> AllowRules {
        AllowRules {
            udp: RangeInclusiveSet::new(),
            tcp: RangeInclusiveSet::new(),
            icmp: false,
        }
    }

    fn apply(&self, packet: &IpPacket) -> Result<(), Filtered> {
        if let Some(dest_port) = packet.as_tcp().map(|tcp| tcp.destination_port()) {
            if self.tcp.contains(&dest_port) {
                return Ok(());
            }

            return Err(Filtered::Tcp);
        }

        if let Some(dest_port) = packet.as_udp().map(|udp| udp.destination_port()) {
            if self.udp.contains(&dest_port) {
                return Ok(());
            }

            return Err(Filtered::Udp);
        }

        if packet.is_icmp() || packet.is_icmpv6() {
            if self.icmp {
                return Ok(());
            }

            return Err(Filtered::Icmp);
        }

        Ok(())
    }

    fn add_filters<'a>(&mut self, filters: impl IntoIterator<Item = &'a Filter>) {
        for filter in filters {
            match filter {
                Filter::Udp(range) => {
                    self.udp
                        .insert(range.port_range_start..=range.port_range_end);
                }
                Filter::Tcp(range) => {
                    self.tcp
                        .insert(range.port_range_start..=range.port_range_end);
                }
                Filter::Icmp => {
                    self.icmp = true;
                }
            }
        }
    }
}
