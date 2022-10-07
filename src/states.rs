use bevy::prelude::*;

use crate::{resources::GameStates, asteroids::asteroid_spawning_system};

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameStates::InGame)
                .with_system(asteroid_spawning_system))
            .add_system_set(SystemSet::on_update(GameStates::InGame))
            .add_system_set(SystemSet::on_exit(GameStates::InGame));
    }
}