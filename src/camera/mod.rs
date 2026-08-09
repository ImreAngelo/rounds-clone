use bevy::prelude::*;

// use crate::player::components::PlayerId;

mod follow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Startup, spawn_camera);
        // app.add_systems(Update, follow::follow_camera_2d);
    }
}

// fn spawn_camera(mut commands: Commands) {
//     commands.spawn((
//         Camera2d,
//         // follow::FollowPlayer(PlayerId(0)),
//         // follow::FollowCamera {
//         //     offset: Vec2::ZERO,
//         //     smoothness: 7.0,
//         //     z: 99.0,
//         // },
//     ));
// }