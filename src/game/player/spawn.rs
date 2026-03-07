use bevy::prelude::*;

use super::{components::*};

/// Spawn players on map. Only spawn players that are not alive.
pub fn spawn_players(
	mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	controllers: Query<(Entity, &PlayerController), (With<PlayerController>, Without<Controls>)>,
) {
	for (controller, player) in &controllers {
		// let x = player.id as f32 * 100.0 - 150.0;

        info!("Spawning player {}", player.id);

		let pawn = commands
			// .spawn((
			// 	Pawn { id: player.id },
			// 	// Transform::from_xyz(x, 0.0, 0.0),
            //     Transform::IDENTITY,
			// 	GlobalTransform::default(),
			// ))
            .spawn((
                Pawn { id: player.id },
                Transform::from_xyz(0.0, 0.0, 0.0),
                Mesh2d(meshes.add(Circle::new(25.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.9, 0.2, 0.2))),
            ))
			.id();

		commands.entity(controller).insert(Controls(pawn));

		info!("spawned pawn for player {}", player.id + 1);
	}
}
