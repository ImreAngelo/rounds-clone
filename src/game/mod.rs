use bevy::prelude::*;

mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()
            .add_plugins(player::PlayerPlugin)
            .add_systems(
                Update, 
                start_game_on_enter
                    .run_if(in_state(AppState::Lobby))
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
fn start_game_on_enter(
	keys: Res<ButtonInput<KeyCode>>,
	mut next_state: ResMut<NextState<AppState>>,
) {
	if keys.just_pressed(KeyCode::Enter) {
        println!("Starting game.");
		next_state.set(AppState::InGame);
	}
}
