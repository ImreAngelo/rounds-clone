use bevy::prelude::*;

mod player;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .run();
}
