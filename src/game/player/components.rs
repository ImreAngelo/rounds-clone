use bevy::prelude::*;
// use bevy_enhanced_input::prelude::*;

/// Used to assign IDs to player controllers
#[derive(Resource, Default)]
pub struct PlayerCount(pub usize);

/// Player component, can move around etc.
#[derive(Component)]
pub struct Pawn {
	pub id: usize,
}

/// Controls a player
#[derive(Component)]
pub struct PlayerController {
	pub id: usize,
}

/// Links controller <-> player
#[derive(Component)]
pub struct Controls(pub Entity); 

// /// Player input actions
// #[derive(InputAction)]
// #[action_output(bool)]
// pub struct Jump;

// /// Horizontal and vertical input
// #[derive(InputAction)]
// #[action_output(Vec2)]
// pub struct Move;
