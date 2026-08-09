use bevy_enhanced_input::prelude::*;
use bevy::prelude::*;

/// Controls player movement
#[derive(InputAction)]
#[action_output(Vec2)]
pub(crate) struct Movement;

/// Jump button pressed
#[derive(InputAction)]
#[action_output(bool)]
pub(crate) struct Jump;

/// Direction of aim, 
/// towards mouse or controller stick  
#[derive(InputAction)]
#[action_output(Vec2)]
pub(crate) struct Aim;

/// Shoot button pressed
#[derive(InputAction)]
#[action_output(bool)]
pub(crate) struct Shoot;