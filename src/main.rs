use bevy::prelude::*;
use bevy_lunex::UiLunexPlugins;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

mod game;
mod camera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiLunexPlugins,
            game::GamePlugin,
            camera::CameraPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
			RapierDebugRenderPlugin::default(),
        ))
        .run();
}