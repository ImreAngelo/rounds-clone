use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use bevy_enhanced_input::prelude::*;

use super::components::*;
use super::bundles::*;

#[derive(InputAction)]
#[action_output(Vec2)]
pub(crate) struct Move;

#[derive(InputAction)]
#[action_output(bool)]
pub(crate) struct Jump;


/// Create a host controller on startup
pub fn setup(
	mut commands: Commands,
	mut next_id: ResMut<PlayerCount>,
) {
	// Player 1 exists in the lobby as a controller only.
	// No pawn yet.
	commands.spawn(host_controller_bundle(next_id.0));

	next_id.0 += 1;
}

/// Create a new player controller when a gamepad connects
pub fn join_on_gamepad_connect(
	mut commands: Commands,
	mut events: MessageReader<GamepadConnectionEvent>,
	controllers: Query<&GamepadDevice, With<PlayerController>>,
	mut next_id: ResMut<PlayerCount>,
) {
	for event in events.read() {
		if !event.connected() { continue; }

		let gamepad = event.gamepad;

		// Prevent the same gamepad from joining twice.
		let already_joined = controllers.iter().any(|device| {
			matches!(device, GamepadDevice::Single(pad) if *pad == gamepad)
		});

		if already_joined {
			continue;
		}

		// Create controller
		let id = next_id.0;
		next_id.0 += 1;

		commands.spawn(gamepad_controller_bundle(id, gamepad));

		info!("player {} joined in lobby", id + 1);
	}
}


// TODO: use tnua
pub fn apply_movement(
	trigger: On<Fire<Move>>,
	controllers: Query<&Controls, With<PlayerController>>,
	mut pawns: Query<&mut Transform, With<Pawn>>,
) {
	let controls = controllers.get(trigger.context).unwrap();
	let mut transform = pawns.get_mut(controls.0).unwrap();
	transform.translation += trigger.value.extend(0.0);
}

// TODO: use tnua
pub fn apply_jump(
	trigger: On<Start<Jump>>,
	controllers: Query<&Controls, With<PlayerController>>,
	mut pawns: Query<&mut Transform, With<Pawn>>,
) {
	let controls = controllers.get(trigger.context).unwrap();
	let mut transform = pawns.get_mut(controls.0).unwrap();
	transform.translation.y += 30.0;
}