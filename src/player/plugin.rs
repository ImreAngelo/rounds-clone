use bevy::prelude::*;
// use leafwing_input_manager::prelude::*;
use bevy_enhanced_input::prelude::*;
use super::{components::*, systems::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(InputManagerPlugin::<Action>::default())
        //     .add_systems(Startup, spawn_player)
        //     .add_systems(Update, jump)
        //     .run();

        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<Player>()
            .add_systems(Startup, spawn_player)
            .add_observer(on_jump);
    }
}