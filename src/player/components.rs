use bevy::prelude::*;
// use leafwing_input_manager::Actionlike;
use bevy_enhanced_input::prelude::*;

/// Player component, can move around etc.
#[derive(Component)]
pub struct Player;

// /// Player input actions
// #[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
// pub enum Action {
// 	Jump    = 0x0,
//     Left    = 0x1,
//     Right   = 0x2,
//     Down    = 0x4,
// }

#[derive(InputAction)]
#[action_output(bool)]
pub struct Jump;