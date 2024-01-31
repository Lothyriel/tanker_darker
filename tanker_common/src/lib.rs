use bevy::{
    prelude::{Bundle, Color, Component, Deref, DerefMut, Vec3},
    time::Timer,
};
use bevy_replicon::{prelude::Replication, renet::ClientId};
use serde::{Deserialize, Serialize};

pub mod events;

pub const PORT: u16 = 42069;
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

#[derive(Component)]
pub struct BombControl(pub Timer);

#[derive(Component, Deserialize, Serialize)]
pub struct BombPosition(pub Vec3);

#[derive(Bundle)]
pub struct BombBundle {
    position: BombPosition,
    color: PlayerColor,
    replication: Replication,
}

impl BombBundle {
    pub fn new(position: Vec3, color: Color) -> Self {
        Self {
            color: PlayerColor(color),
            position: BombPosition(position),
            replication: Replication,
        }
    }
}
