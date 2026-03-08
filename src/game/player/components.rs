use bevy::prelude::*;
use bevy_tnua::TnuaScheme;
use bevy_tnua::builtins::{TnuaBuiltinJump, TnuaBuiltinWalk};

/// The host takes control of the keyboard and first connected controller by default
#[derive(Component)]
pub(crate) struct HostKeyboardController;

/// Used to assign IDs to player controllers
#[derive(Resource, Default)]
pub(crate) struct PlayerCount(pub(crate) usize);

/// Player component, can move around etc.
#[derive(Component)]
pub(crate) struct Pawn {
	pub(crate) id: usize,
}

/// Controls a player
#[derive(Component)]
pub(crate) struct PlayerController {
	id: usize,
}

impl PlayerController {
	pub(crate) fn new(id: usize) -> Self {
		Self { id }
	}

	pub(crate) fn id(&self) -> usize {
		self.id
	}
}

/// Links controller <-> player
#[derive(Component)]
pub(crate) struct Controls(pub(crate) Entity);

/// Tnua control scheme for the player
#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub(crate) enum PlayerScheme {
    Jump(TnuaBuiltinJump),
}