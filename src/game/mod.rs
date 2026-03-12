use bevy::{camera::visibility::RenderLayers, prelude::*};
use bevy_lunex::UiSourceCamera;

mod player;
mod map;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()
            .add_plugins((
                player::PlayerPlugin,
                map::MapPlugin
            ))
            .add_systems(
                Update, 
                toggle_state
            )
            .add_systems(
                Startup,
                spawn_camera
            );
    }
}



/// Game State
#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum AppState {
	#[default]
	Lobby,
	InGame,
}

/// DEBUG: Switch game state
fn toggle_state(
	keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
	mut next_state: ResMut<NextState<AppState>>,
) {
	if !keys.just_pressed(KeyCode::Enter) { return; }
    
    match current_state.get() {
        AppState::Lobby => {
            println!("Starting game.");
            next_state.set(AppState::InGame);
        },
        AppState::InGame => {
            println!("Returning to lobby.");
            next_state.set(AppState::Lobby);
        }
        // _ => {
        //     println!("Unknown state/transition.")
        // }
    }
}


/// DEBUG: Spawn UI camera
fn spawn_camera(mut commands: Commands) {
    // Spawn the camera
    commands.spawn((

        // This camera will become the source for all UI paired to index 0.
        Camera2d, UiSourceCamera::<0>,
        
        // Ui nodes start at 0 and move + on the Z axis with each depth layer.
        // This will ensure you will see up to 1000 nested children.
        Transform::from_translation(Vec3::Z * 1000.0),
        
        // Explained in # Chapters/Debug-Tooling section of the book
        RenderLayers::from_layers(&[0, 1]),
    ));
}