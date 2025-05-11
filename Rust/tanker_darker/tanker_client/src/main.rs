use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;

use bevy_replicon::{
    prelude::*,
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ConnectionConfig,
    },
};

use tanker_common::{events::Register, *};

mod graphics;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ReplicationPlugins.build().disable::<ServerPlugin>())
        .add_plugins(ClientPlugin)
        .run();
}

struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.register_replications()
            .register_events()
            .add_plugins(graphics::GraphicsPlugin)
            .add_plugins(input::InputPlugin)
            .add_systems(PreStartup, Self::connect_server_system.map(Result::unwrap));
    }
}

impl ClientPlugin {
    fn connect_server_system(
        mut commands: Commands,
        network_channels: Res<NetworkChannels>,
    ) -> anyhow::Result<()> {
        let server_channels_config = network_channels.get_server_configs();
        let client_channels_config = network_channels.get_client_configs();

        let client = RenetClient::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            ..Default::default()
        });

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        let client_id = current_time.as_millis() as u64;

        let server_ip = dotenvy_macro::dotenv!("SERVER_ADDR").parse()?;

        let server_addr = SocketAddr::new(server_ip, PORT);

        info!("Connecting on {}", server_addr);

        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: None,
        };

        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;

        let transport = NetcodeClientTransport::new(current_time, authentication, socket)?;

        commands.insert_resource(client);
        commands.insert_resource(transport);

        Ok(())
    }
}
