use bevy::prelude::*;

use bevy_replicon::{prelude::*, renet::ServerEvent};
use tanker_common::{events::MoveDirection, Player, PlayerBundle, PlayerPosition};

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (Self::movement_system, Self::server_event_system),
        );
    }
}

impl TickPlugin {
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

                    let color = Color::rgb(r, g, b);

                    commands.spawn(PlayerBundle::new(*client_id, Vec3::ZERO, color));
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
        const MOVE_SPEED: f32 = 3.0;

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
