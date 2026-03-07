// use bevy::prelude::*;

// use crate::player::components::Player;

// #[derive(Component)]
// pub struct FollowPlayer(pub u8);

// #[derive(Component)]
// pub struct FollowCamera {
//     pub offset: Vec2,      // XY offset in world units
//     pub smoothness: f32,   // higher = snappier
//     pub z: f32,            // camera Z
// }

// /// Follow player
// pub fn follow_camera_2d(
//     time: Res<Time>,
//     players: Query<(&Player, &GlobalTransform)>,
//     mut cams: Query<(&FollowPlayer, &FollowCamera, &mut Transform), With<Camera>>,
// ) {
//     for (follow_player, cam_cfg, mut cam_tf) in &mut cams {
//         // Find the target with the requested id
//         let target_xy = players
//             .iter()
//             .find(|(pid, _)| **pid == follow_player.0)
//             .map(|(_, gt)| gt.translation().truncate());

//         let Some(target_xy) = target_xy else { continue; };

//         let desired_xy = target_xy + cam_cfg.offset;

//         // Exponential smoothing (frame-rate independent)
//         let t = 1.0 - (-cam_cfg.smoothness * time.delta_secs()).exp();
//         let current_xy = cam_tf.translation.truncate();
//         let new_xy = current_xy.lerp(desired_xy, t);

//         cam_tf.translation = new_xy.extend(cam_cfg.z);
//     }
// }