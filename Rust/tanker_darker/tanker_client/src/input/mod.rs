use bevy::{input::mouse::MouseWheel, prelude::*};
use tanker_common::events::{MoveDirection, SpawnBomb};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, Self::movement_system)
            .add_systems(Update, Self::camera_zoom_system)
            .add_systems(Update, Self::bomb_system);
    }
}

impl InputPlugin {
    /// Reads player inputs and sends [`MoveCommandEvents`]
    fn movement_system(mut move_events: EventWriter<MoveDirection>, input: Res<Input<KeyCode>>) {
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

    fn bomb_system(mut move_events: EventWriter<SpawnBomb>, input: Res<Input<KeyCode>>) {
        if input.pressed(KeyCode::Space) {
            move_events.send(SpawnBomb);
        }
    }

    fn camera_zoom_system(
        mut scroll_evts: EventReader<MouseWheel>,
        mut query: Query<&mut Transform, With<Camera>>,
    ) {
        let mut transform = query.single_mut();

        let scroll: f32 = scroll_evts.read().map(|e| e.y).sum();

        transform.translation.z += scroll;
    }
}
