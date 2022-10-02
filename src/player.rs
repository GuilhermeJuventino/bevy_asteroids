use bevy::prelude::*;

use crate::{
    components::{Player, Rotate},
    constants::PLAYER_ROTATION_SPEED,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_keyboard_event_system)
            .add_system(player_movement_system);
    }
}

// player systems
fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Rotate, With<Player>>,
) {
    if let Ok(mut rotate) = query.get_single_mut() {
        rotate.z = if kb.pressed(KeyCode::Left) {
            PLAYER_ROTATION_SPEED
        } else if kb.pressed(KeyCode::Right) {
            -PLAYER_ROTATION_SPEED
        } else {
            0.
        }
    }
}

fn player_movement_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Rotate, &mut Transform), With<Player>>,
) {
    for (entity, rotate, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let rotation = &mut transform.rotation;
        rotation.z += rotate.z;
    }
}
