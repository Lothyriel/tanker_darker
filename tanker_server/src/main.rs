use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{log::LogPlugin, prelude::*};

use bevy_replicon::{
    prelude::*,
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, ServerEvent,
    },
};

use tanker_common::*;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, ReplicationPlugins, ServerPlugin))
        .add_plugins(LogPlugin::default())
        .run();
}

struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<PlayerPosition>()
            .replicate::<PlayerColor>()
            .add_client_event::<MoveDirection>(EventType::Ordered)
            .add_systems(Startup, Self::init_system.map(Result::unwrap))
            .add_systems(
                FixedUpdate,
                (Self::movement_system, Self::server_event_system),
            );
    }
}

impl ServerPlugin {
    fn init_system(
        mut commands: Commands,
        network_channels: Res<NetworkChannels>,
    ) -> Result<(), Box<dyn Error>> {
        let server_channels_config = network_channels.get_server_configs();
        let client_channels_config = network_channels.get_client_configs();

        let server = RenetServer::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            ..Default::default()
        });

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), PORT);
        let socket = UdpSocket::bind(public_addr)?;
        let server_config = ServerConfig {
            current_time,
            max_clients: 10,
            protocol_id: PROTOCOL_ID,
            authentication: ServerAuthentication::Unsecure,
            public_addresses: vec![public_addr],
        };
        let transport = NetcodeServerTransport::new(server_config, socket)?;

        commands.insert_resource(server);
        commands.insert_resource(transport);

        Ok(())
    }

    /// Logs server events and spawns a new player whenever a client connects.
    fn server_event_system(
        mut commands: Commands,
        mut server_event: EventReader<ServerEvent>,
        mut query: Query<(Entity, &Player)>,
    ) {
        for event in server_event.read() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    info!("player: {client_id} Connected");
                    // Generate pseudo random color from client id.
                    let r = ((client_id.raw() % 23) as f32) / 23.0;
                    let g = ((client_id.raw() % 27) as f32) / 27.0;
                    let b = ((client_id.raw() % 39) as f32) / 39.0;
                    commands.spawn(PlayerBundle::new(
                        *client_id,
                        Vec2::ZERO,
                        Color::rgb(r, g, b),
                    ));
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    info!("client {client_id} disconnected: {reason}");
                    for (entity, player) in &mut query {
                        if *client_id == player.0 {
                            commands.entity(entity).despawn()
                        }
                    }
                }
            }
        }
    }

    /// Mutates [`PlayerPosition`] based on [`MoveCommandEvents`].
    ///
    /// Fast-paced games usually you don't want to wait until server send a position back because of the latency.
    /// But this example just demonstrates simple replication concept.
    fn movement_system(
        time: Res<Time>,
        mut move_events: EventReader<FromClient<MoveDirection>>,
        mut players: Query<(&Player, &mut PlayerPosition)>,
    ) {
        const MOVE_SPEED: f32 = 300.0;
        for FromClient { client_id, event } in move_events.read() {
            info!("received event {event:?} from client {client_id}");
            for (player, mut position) in &mut players {
                if *client_id == player.0 {
                    **position += event.0 * time.delta_seconds() * MOVE_SPEED;
                }
            }
        }
    }
}
