use std::{
    error::Error,
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
use tanker_common::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ReplicationPlugins, ClientPlugin))
        .run();
}

struct ClientPlugin;

impl ClientPlugin {
    fn init_system(
        mut commands: Commands,
        network_channels: Res<NetworkChannels>,
    ) -> Result<(), Box<dyn Error>> {
        let server_channels_config = network_channels.get_server_configs();
        let client_channels_config = network_channels.get_client_configs();

        let client = RenetClient::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            ..Default::default()
        });

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let client_id = current_time.as_millis() as u64;

        let server_ip = Ipv4Addr::LOCALHOST.into();
        let server_addr = SocketAddr::new(server_ip, PORT);
        let socket = UdpSocket::bind((server_ip, 0))?;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: None,
        };
        let transport = NetcodeClientTransport::new(current_time, authentication, socket)?;

        commands.insert_resource(client);
        commands.insert_resource(transport);

        commands.spawn(TextBundle::from_section(
            format!("Client: {client_id:?}"),
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        commands.spawn(Camera2dBundle::default());

        Ok(())
    }

    fn draw_boxes_system(mut gizmos: Gizmos, players: Query<(&PlayerPosition, &PlayerColor)>) {
        for (position, color) in &players {
            gizmos.rect(
                Vec3::new(position.x, position.y, 0.0),
                Quat::IDENTITY,
                Vec2::ONE * 50.0,
                color.0,
            );
        }
    }

    /// Reads player inputs and sends [`MoveCommandEvents`]
    fn input_system(mut move_events: EventWriter<MoveDirection>, input: Res<Input<KeyCode>>) {
        let mut direction = Vec2::ZERO;
        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        if direction != Vec2::ZERO {
            move_events.send(MoveDirection(direction.normalize_or_zero()));
        }
    }
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<PlayerPosition>()
            .replicate::<PlayerColor>()
            .add_client_event::<MoveDirection>(EventType::Ordered)
            .add_systems(Startup, Self::init_system.map(Result::unwrap))
            .add_systems(Update, (Self::draw_boxes_system, Self::input_system));
    }
}
