use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use bevy_enhanced_input::prelude::*;

use super::{components::*};

#[derive(InputAction)]
#[action_output(Vec2)]
pub(crate) struct Move;

#[derive(InputAction)]
#[action_output(bool)]
pub(crate) struct Jump;



pub fn setup(
	mut commands: Commands,
	mut next_id: ResMut<PlayerCount>,
) {
	// Player 1 exists in the lobby as a controller only.
	// No pawn yet.
	commands.spawn((
		PlayerController { id: next_id.0 },
		// HostKeyboardController,
		GamepadDevice::None,
		actions!(PlayerController[
			(
				Action::<Move>::new(),
				DeadZone::default(),
				DeltaScale::default(),
				Scale::splat(300.0),
				Bindings::spawn((
					Cardinal::wasd_keys(),
					Axial::left_stick(),
				)),
			),
			(
				Action::<Jump>::new(),
				bindings![
					KeyCode::Space,
					GamepadButton::South,
				],
			),
		]),
	));

	next_id.0 += 1;
}


pub fn join_on_gamepad_connect(
	mut commands: Commands,
	mut events: MessageReader<GamepadConnectionEvent>,
	controllers: Query<&GamepadDevice, With<PlayerController>>,
	mut next_id: ResMut<PlayerCount>,
) {
	for event in events.read() {
		if !event.connected() {
			continue;
		}

		let gamepad = event.gamepad;

		// Prevent the same gamepad from joining twice.
		let already_joined = controllers.iter().any(|device| {
			matches!(device, GamepadDevice::Single(pad) if *pad == gamepad)
		});

		if already_joined {
			continue;
		}

		let id = next_id.0;
		next_id.0 += 1;

		commands.spawn((
			PlayerController { id },
			GamepadDevice::Single(gamepad),
			actions!(PlayerController[
				(
					Action::<Move>::new(),
					DeadZone::default(),
					DeltaScale::default(),
					Scale::splat(300.0),
					Bindings::spawn(Axial::left_stick()),
				),
				(
					Action::<Jump>::new(),
					bindings![GamepadButton::South],
				),
			]),
		));

		info!("player {} joined in lobby", id + 1);
	}
}


// pub fn setup(
// 	mut commands: Commands,
// 	mut meshes: ResMut<Assets<Mesh>>,
// 	mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
// 	// Pawn 1
// 	let pawn1 = commands
// 		.spawn((
// 			Pawn { id: 0 },
// 			Transform::from_xyz(-120.0, 0.0, 0.0),
// 			Mesh2d(meshes.add(Circle::new(25.0))),
// 			MeshMaterial2d(materials.add(Color::srgb(0.9, 0.2, 0.2))),
// 		))
// 		.id();

// 	// Controller 1: keyboard by default, gamepad can be attached later too.
// 	commands.spawn((
// 		PlayerController { id: 0 },
// 		Controls(pawn1),
// 		GamepadDevice::None,
// 		actions!(PlayerController[
// 			(
// 				Action::<Move>::new(),
// 				DeadZone::default(),
// 				DeltaScale::default(),
// 				Scale::splat(300.0),
// 				Bindings::spawn((
//                     Cardinal::wasd_keys(),
//                     Axial::left_stick(),
//                 )),
// 			),
// 			(
// 				Action::<Jump>::new(),
// 				bindings![
// 					KeyCode::Space,
// 					GamepadButton::South,
// 				],
// 			),
// 		]),
// 	));

// 	// Pawn 2
// 	let pawn2 = commands
// 		.spawn((
// 			Pawn { id: 1 },
// 			Transform::from_xyz(120.0, 0.0, 0.0),
// 			Mesh2d(meshes.add(Circle::new(25.0))),
// 			MeshMaterial2d(materials.add(Color::srgb(0.2, 0.4, 0.9))),
// 		))
// 		.id();

// 	// Controller 2: no keyboard, waiting for a gamepad
// 	commands.spawn((
// 		PlayerController { id: 1 },
// 		Controls(pawn2),
// 		GamepadDevice::None, 
// 		actions!(PlayerController[
// 			(
// 				Action::<Move>::new(),
// 				DeadZone::default(),
// 				DeltaScale::default(),
// 				Scale::splat(300.0),
// 				Bindings::spawn((
//                     Axial::left_stick(),
//                 )),
// 			),
// 			(
// 				Action::<Jump>::new(),
// 				bindings![
//                     GamepadButton::South,
//                 ],
// 			),
// 		]),
// 	));
// }

// pub fn assign_gamepads_to_free_controllers(
// 	mut events: MessageReader<GamepadConnectionEvent>,
// 	mut controllers: Query<(Entity, &PlayerController, &mut GamepadDevice)>,
// ) {
// 	for event in events.read() {
// 		if !event.connected() {
// 			continue;
// 		}

// 		let gamepad = event.gamepad;

// 		// Ignore if already assigned
// 		if controllers
// 			.iter()
// 			.any(|(_, _, device)| matches!(*device, GamepadDevice::Single(pad) if pad == gamepad))
// 		{
// 			continue;
// 		}

// 		// Attach to the first free slot
// 		for (_, controller, mut device) in &mut controllers {
// 			if matches!(*device, GamepadDevice::None) {
// 				*device = GamepadDevice::Single(gamepad);
// 				info!("assigned {:?} to player {}", gamepad, controller.id + 1);
// 				break;
// 			}
// 		}
// 	}
// }

pub fn apply_movement(
	trigger: On<Fire<Move>>,
	controllers: Query<&Controls, With<PlayerController>>,
	mut pawns: Query<&mut Transform, With<Pawn>>,
) {
	let controls = controllers.get(trigger.context).unwrap();
	let mut transform = pawns.get_mut(controls.0).unwrap();
	transform.translation += trigger.value.extend(0.0);
}

pub fn apply_jump(
	trigger: On<Start<Jump>>,
	controllers: Query<&Controls, With<PlayerController>>,
	mut pawns: Query<&mut Transform, With<Pawn>>,
) {
	let controls = controllers.get(trigger.context).unwrap();
	let mut transform = pawns.get_mut(controls.0).unwrap();
	transform.translation.y += 30.0;
}