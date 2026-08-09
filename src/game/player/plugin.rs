use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_rapier2d::TnuaRapier2dPlugin;

use crate::game::AppState;

use super::{components::*, systems::*, spawn::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EnhancedInputPlugin,
                TnuaControllerPlugin::<PlayerScheme>::new(FixedUpdate),
                TnuaRapier2dPlugin::new(FixedUpdate),
            ))
            .add_input_context::<PlayerController>()
            .init_resource::<PlayerCount>()
            .add_systems(Update, apply_movement.in_set(TnuaUserControlsSystems))
            .add_systems(Update, update_hand)
            .add_observer(shoot_weapon)
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