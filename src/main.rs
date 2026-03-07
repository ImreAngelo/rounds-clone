use bevy::prelude::*;
use bevy_lunex::UiLunexPlugins;

mod game;
mod camera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiLunexPlugins,
            game::GamePlugin,
            camera::CameraPlugin,
        ))
        .run();
}