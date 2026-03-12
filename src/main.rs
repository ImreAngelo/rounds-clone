use bevy::prelude::*;
use bevy_lunex::UiLunexPlugins;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use bevy_steamworks::SteamworksPlugin;

mod game;
mod camera;

fn main() {
    const STEAM_APP_ID : u32 = 480;

    let Ok(steamworks) = SteamworksPlugin::init_app(STEAM_APP_ID) else {
        error!("Error with initializing Steamworks SDK");
        return;
    };

    App::new()
        .add_plugins(steamworks)
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