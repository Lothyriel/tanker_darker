use bevy::{input::mouse::MouseWheel, prelude::*};
use tanker_common::events::MoveDirection;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, Self::input_system);
    }
}

impl InputPlugin {
    /// Reads player inputs and sends [`MoveCommandEvents`]
    fn input_system(mut move_events: EventWriter<MoveDirection>, input: Res<Input<KeyCode>>) {
        let mut direction = Vec3::ZERO;

        let pressed = |k| if input.pressed(k) { 1.0 } else { 0.0 };

        direction.x += pressed(KeyCode::Right);
        direction.x -= pressed(KeyCode::Left);
        direction.z -= pressed(KeyCode::Up);
        direction.z += pressed(KeyCode::Down);

        if direction != Vec3::ZERO {
            move_events.send(MoveDirection(direction.normalize_or_zero()));
        }
    }
}
