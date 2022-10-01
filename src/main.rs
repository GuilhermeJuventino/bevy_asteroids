use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use resources::WinSize;

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
}
