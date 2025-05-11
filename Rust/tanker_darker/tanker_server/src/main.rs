use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{log::LogPlugin, prelude::*};

use bevy_replicon::{
    prelude::*,
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig,
    },
};

use tanker_common::{events::Register, *};

mod tick;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(ReplicationPlugins.build().disable::<ClientPlugin>())
        .add_plugins(LogPlugin::default())
        .add_plugins(ServerPlugin)
        .run();
}

struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.register_replications()
            .register_events()
            .add_plugins(tick::TickPlugin)
            .add_systems(Startup, Self::init_system.map(Result::unwrap));
    }
}

impl ServerPlugin {
    fn init_system(
        mut commands: Commands,
        network_channels: Res<NetworkChannels>,
    ) -> anyhow::Result<()> {
        let server_channels_config = network_channels.get_server_configs();
        let client_channels_config = network_channels.get_client_configs();

        let server = RenetServer::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            ..Default::default()
        });

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        let public_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), PORT);

        info!("Starting server on: {}", public_addr);

        let server_config = ServerConfig {
            current_time,
            max_clients: 30,
            protocol_id: PROTOCOL_ID,
            authentication: ServerAuthentication::Unsecure,
            public_addresses: vec![public_addr],
        };

        let socket = UdpSocket::bind(public_addr)?;

        let transport = NetcodeServerTransport::new(server_config, socket)?;

        commands.insert_resource(server);
        commands.insert_resource(transport);

        Ok(())
    }
}
