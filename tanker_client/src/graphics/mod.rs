use bevy::prelude::*;
use bevy_replicon::renet::transport::NetcodeClientTransport;
use tanker_common::{PlayerColor, PlayerPosition};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, Self::init_ui_system)
            .add_systems(Update, Self::draw_boxes_system);
    }
}

impl GraphicsPlugin {
    fn init_ui_system(mut commands: Commands, transport: Res<NetcodeClientTransport>) {
        commands.spawn(TextBundle::from_section(
            format!("Client: {}", transport.client_id()),
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        commands.spawn(Camera2dBundle::default());
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
}
