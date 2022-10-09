use bevy::prelude::*;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub big_asteroid: Handle<Image>,
    pub med_asteroid: Handle<Image>,
    pub small_asteroid: Handle<Image>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameStates {
    TitleScreen,
    InGame,
    GameOver,
}

// player resources
