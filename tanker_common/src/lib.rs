use bevy::prelude::{Bundle, Color, Component, Deref, DerefMut, Vec3};
use bevy_replicon::{prelude::Replication, renet::ClientId};
use serde::{Deserialize, Serialize};

pub mod events;
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
    pub fn new(client_id: ClientId, position: Vec3, color: Color) -> Self {
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
pub struct PlayerPosition(pub Vec3);

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerColor(pub Color);
