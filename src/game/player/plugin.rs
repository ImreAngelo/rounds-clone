use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::game::AppState;

use super::{components::*, systems::*, spawn::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EnhancedInputPlugin)
            .add_input_context::<PlayerController>()
            .init_resource::<PlayerCount>()
            .add_observer(apply_movement)
            .add_observer(apply_jump)
            .add_systems(Startup, setup)
            .add_systems(
                Update, 
                (
                    join_on_gamepad_connect
                )
                .run_if(in_state(AppState::Lobby))
            )
            .add_systems( // TODO: Does this really belong here?
                OnEnter(AppState::InGame), 
                spawn_players
            );
    }
}