use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::AppState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame), 
            spawn_debug_map
        );
    }
}

fn spawn_debug_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Spawning map 1");
    let half_width = 512.0;
    let half_height = 32.0;

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(half_width, half_height),
        Friction::coefficient(0.7),
        Restitution::coefficient(0.3),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Mesh2d(meshes.add(Rectangle::new(half_width * 2.0, half_height * 2.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.2, 0.0))),
    ));
}