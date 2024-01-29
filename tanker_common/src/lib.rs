use bevy::prelude::{Bundle, Color, Component, Deref, DerefMut, Event, Vec2};
use bevy_replicon::{prelude::Replication, renet::ClientId};
use serde::{Deserialize, Serialize};

pub mod infra;

pub const PORT: u16 = 6969;
pub const PROTOCOL_ID: u64 = 0;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    position: PlayerPosition,
    color: PlayerColor,
    replication: Replication,
}

impl PlayerBundle {
    pub fn new(client_id: ClientId, position: Vec2, color: Color) -> Self {
        Self {
            player: Player(client_id),
            position: PlayerPosition(position),
            color: PlayerColor(color),
            replication: Replication,
        }
    }
}

/// Contains the client ID of the player.
#[derive(Component, Serialize, Deserialize)]
pub struct Player(pub ClientId);

#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct PlayerPosition(pub Vec2);

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerColor(pub Color);

/// A movement event for the controlled box.
#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection(pub Vec2);
