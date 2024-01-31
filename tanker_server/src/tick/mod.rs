use bevy::prelude::*;

use bevy_replicon::{prelude::*, renet::ServerEvent};
use tanker_common::{
    events::{MoveDirection, SpawnBomb},
    BombBundle, Player, PlayerBundle, PlayerColor, PlayerPosition,
};

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, Self::movement_system)
            .add_systems(FixedUpdate, Self::server_event_system)
            .add_systems(FixedUpdate, Self::spawn_bombs_system);
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
                    info!("Player: {client_id} Connected");
                    // Generate pseudo random color from client id.
                    let r = ((client_id.raw() % 23) as f32) / 23.0;
                    let g = ((client_id.raw() % 27) as f32) / 27.0;
                    let b = ((client_id.raw() % 39) as f32) / 39.0;

                    let color = Color::rgb(r, g, b);

                    commands.spawn(PlayerBundle::new(*client_id, Vec3::new(0., 0.5, 0.), color));
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    info!("Player {client_id} disconnected: {reason}");

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
        mut events: EventReader<FromClient<MoveDirection>>,
        mut query: Query<(&Player, &mut PlayerPosition)>,
    ) {
        const MOVE_SPEED: f32 = 3.0;

        for FromClient { client_id, event } in events.read() {
            for (player, mut position) in &mut query {
                if *client_id == player.0 {
                    **position += event.0 * time.delta_seconds() * MOVE_SPEED;
                }
            }
        }
    }

    fn spawn_bombs_system(
        mut commands: Commands,
        mut events: EventReader<FromClient<SpawnBomb>>,
        mut query: Query<(&Player, &PlayerPosition, &PlayerColor)>,
    ) {
        for FromClient {
            client_id,
            event: _,
        } in events.read()
        {
            for (player, position, color) in &mut query {
                if *client_id == player.0 {
                    commands.spawn(BombBundle::new(position.0, color.0));
                    info!("Player: {} Spawned bomb at {}", player.0, position.0);
                }
            }
        }
    }
}
