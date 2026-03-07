use bevy::prelude::*;

mod game;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game::GamePlugin)
        .add_plugins(camera::CameraPlugin)
        .run();
}