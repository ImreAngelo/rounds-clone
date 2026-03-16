use bevy::prelude::*;

use super::bundles::*;
use super::components::*;

const HAND_RADIUS: f32 = 41.0;

/// Spawn players on map. Only spawn players that are not alive.
pub fn spawn_players(
	mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut scheme_configs: ResMut<Assets<PlayerSchemeConfig>>,
	controllers: Query<(Entity, &PlayerController), (With<PlayerController>, Without<Controls>)>,
) {
	for (controller, player) in &controllers {
		let x = player.id() as f32 * 100.0 - 150.0;

		let pawn = commands
            .spawn((
                Pawn { id: player.id() },
                Transform::from_xyz(x, 0.0, 0.0),
                Mesh2d(meshes.add(Circle::new(25.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.9, 0.2, 0.2))),
                pawn_physics_bundle(&mut scheme_configs),
            ))
			.with_children(|parent| {
				parent.spawn(( // DEBUG: Show arm target
					Mesh2d(meshes.add(Annulus::new(40.0, 42.0))),
					MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
				));
				parent.spawn(( // DEBUG: Hand
					Hand,
					Transform::from_xyz(0.0, HAND_RADIUS, 1.0),
					Mesh2d(meshes.add(Circle::new(5.0))),
					MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
				));
			})
			.id();

		commands.entity(controller).insert(Controls(pawn));

		info!("Spawned pawn for player {}", player.id() + 1);
	}
}
