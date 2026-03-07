use bevy::prelude::*;
// use leafwing_input_manager::prelude::*;
use bevy_enhanced_input::prelude::*;

use super::{components::*};

pub fn spawn_player(mut commands: Commands) {
	commands.spawn((
		Player,
		Transform::default(),
		GlobalTransform::default(),
        // Action-input map
		actions!(Player[
			(
				Action::<Jump>::new(),
				bindings![KeyCode::Space, KeyCode::KeyW, KeyCode::ArrowUp, GamepadButton::South],
			),
		]),
        // Visuals
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(32.0)),
	));
}


pub fn on_jump(jump: On<Start<Jump>>, mut players: Query<&mut Transform, With<Player>>) {
	let Ok(mut transform) = players.get_mut(jump.context) else {
		return;
	};

	transform.translation.y += 10.0;
}