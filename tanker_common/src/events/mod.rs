use bevy::prelude::{Event, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection(pub Vec3);
