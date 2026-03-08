use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use bevy_enhanced_input::action::relationship::ActionOf;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;

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


pub fn apply_controls(
	controllers: Query<(Entity, &Controls), With<PlayerController>>,
	move_actions: Query<(&Action<Move>, &ActionOf<PlayerController>)>,
	jump_actions: Query<(&TriggerState, &ActionOf<PlayerController>), With<Action<Jump>>>,
	mut pawns: Query<&mut TnuaController<PlayerScheme>, With<Pawn>>,
) {
	for (controller_entity, controls) in &controllers {
		let Ok(mut controller) = pawns.get_mut(controls.0) else { continue };
		controller.initiate_action_feeding();

		let action_of = ActionOf::new(controller_entity);

		let movement = move_actions
			.iter()
			.find(|(_, of)| **of == action_of)
			.map(|(action, _)| action.extend(0.0))
			.unwrap_or(Vec3::ZERO);

		controller.basis = TnuaBuiltinWalk {
			desired_motion: movement,
			..Default::default()
		};

		let jumping = jump_actions
			.iter()
			.any(|(state, of)| *of == action_of && *state == TriggerState::Fired);

		if jumping {
			controller.action(PlayerScheme::Jump(Default::default()));
		}
	}
}