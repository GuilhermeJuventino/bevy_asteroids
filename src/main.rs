use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use components::{Player, SpriteSize, Velocity};
use constants::*;
use resources::{GameTextures, WinSize};

mod components;
mod constants;
mod player;
mod resources;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Asteroids".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());

    // capture windows size
    let window = windows
        .get_primary_mut()
        .expect("Primary window does not exist");
    let (win_w, win_h) = (window.width(), window.height());

    // poisition window
    window.set_position(IVec2::new(660 / 2, 120));

    // add WinSize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        big_asteroid: asset_server.load(BIG_ASTEROID_SPRITE),
        med_asteroid: asset_server.load(MED_ASTEROID_SPRITE),
        small_asteroid: asset_server.load(SMALL_ASTEROID_SPRITE),
        tiny_asteroid: asset_server.load(TINY_ASTEROID_SPRITE),
    };
    commands.insert_resource(game_textures);

    // spawn the player
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(SpriteSize::from(PLAYER_SIZE))
        .insert(Velocity { x: 0., y: 0.});
}
