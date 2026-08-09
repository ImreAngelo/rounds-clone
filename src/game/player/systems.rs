use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use bevy_enhanced_input::action::relationship::ActionOf;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;

use super::components::*;
use super::bundles::*;
use super::inputs::*;

const HAND_RADIUS: f32 = 41.0;


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

pub fn apply_movement(
	controllers: Query<(Entity, &Controls), With<PlayerController>>,
	move_actions: Query<(&Action<Movement>, &ActionOf<PlayerController>)>,
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

///
pub fn shoot_weapon(
	shoot: On<Fire<Shoot>>,
	query: Query<(Entity, &Controls), With<PlayerController>>,
	// aim_actions: Query<(&Action<Aim>, &ActionOf<PlayerController>)>,
	// pawns: Query<(&GlobalTransform, &Children), With<Pawn>>,
	// hands: Query<&mut Transform, With<Hand>>,
) {
	if let Ok(controller) = query.get(shoot.context) {
		info!("Player {} fired their gun!", controller.0);
	};
	

	// for (controller_entity, controls) in &controllers {
	// 	// let Ok((pawn_transform, children)) = pawns.get(controls.0) else { continue };
		
	// 	// let action_of = ActionOf::new(controller_entity);

	// 	// for press in shoot_actions {
	// 	// 	// info!("{}", press.0);
	// 	// }
	// }
}


/// Update the position of the players gun
/// The gun is always X units away from the player, and points towards
/// the players aim direction (from controller stick of mouse cursor)
pub fn update_hand(
	controllers: Query<(Entity, &Controls), With<PlayerController>>,
	aim_actions: Query<(&Action<Aim>, &ActionOf<PlayerController>)>,
	pawns: Query<(&GlobalTransform, &Children), With<Pawn>>,
	mut hands: Query<&mut Transform, With<Hand>>,
	windows: Query<&Window>,
	cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
	let mouse_world = windows.single().ok()
		.and_then(|w| w.cursor_position())
		.and_then(|cursor| {
			let (cam, cam_tf) = cameras.single().ok()?;
			cam.viewport_to_world_2d(cam_tf, cursor).ok()
		});

	for (controller_entity, controls) in &controllers {
		let Ok((pawn_gtf, children)) = pawns.get(controls.0) else { continue };

		let action_of = ActionOf::new(controller_entity);
		let aim = aim_actions
			.iter()
			.find(|(_, of)| **of == action_of)
			.map(|(action, _)| Vec2::new(action.x, action.y));

		let pawn_pos = pawn_gtf.translation().truncate();

		let direction = 'dir: {
			// Right stick with meaningful deflection → gamepad aim
			if let Some(stick) = aim {
				if stick.length_squared() > 0.01 {
					break 'dir stick.normalize();
				}
			}
			// Fallback: mouse cursor projected onto the circle
			if let Some(mouse) = mouse_world {
				let delta = mouse - pawn_pos;
				if delta.length_squared() > 0.001 {
					break 'dir delta.normalize();
				}
			}
			Vec2::Y
		};

		for &child in children {
			if let Ok(mut hand_tf) = hands.get_mut(child) {
				hand_tf.translation = (direction * HAND_RADIUS).extend(1.0);
				break;
			}
		}
	}
}