defmodule FzHttp.Configurations.Cache do
  @moduledoc """
  Manipulate cached configurations.
  """

  use GenServer, restart: :transient

  alias FzHttp.Configurations, as: Conf

  @name :conf

  def get(key) do
    Cachex.get(@name, key)
  end

  def get!(key) do
    Cachex.get!(@name, key)
  end

  def put(key, value) do
    Cachex.put(@name, key, value)
  end

  def put!(key, value) do
    Cachex.put!(@name, key, value)
  end

  def start_link(_) do
    GenServer.start_link(__MODULE__, [])
  end

  @no_fallback [
    :logo,
    :default_client_allowed_ips,
    :default_client_dns,
    :default_client_endpoint,
    :default_client_mtu,
    :default_client_persistent_keepalive,
    :default_client_port,
    :ipv4_enabled,
    :ipv6_enabled,
    :ipv4_network,
    :ipv6_network,
    :vpn_session_duration
  ]

  @impl true
  def init(_) do
    configurations =
      Conf.get_configuration!()
      |> Map.from_struct()
      |> Map.delete(:id)

    for {k, v} <- configurations do
      # XXX: Remove fallbacks before 1.0?
      v =
        with nil <- v, true <- k not in @no_fallback do
          Application.fetch_env!(:fz_http, k)
        else
          _ -> v
        end

      {:ok, _} = put(k, v)
    end

    :ignore
  end
end
