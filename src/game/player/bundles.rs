use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua::builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig};

use super::components::*;
use super::inputs::*;

/// Physics and Tnua components shared by all pawns
pub fn pawn_physics_bundle(scheme_configs: &mut Assets<PlayerSchemeConfig>) -> impl Bundle {
    (
        RigidBody::Dynamic,
        Collider::ball(25.0),
        LockedAxes::ROTATION_LOCKED,
        TnuaController::<PlayerScheme>::default(),
        TnuaConfig::<PlayerScheme>(scheme_configs.add(PlayerSchemeConfig {
            basis: TnuaBuiltinWalkConfig {
                float_height: 64.0,
                speed: 128.0,
                acceleration: 2000.0,
                air_acceleration: 1000.0,
                ..Default::default()
            },
            jump: TnuaBuiltinJumpConfig {
                height: 128.0,
                ..Default::default()
            },
        })),
    )
}

/// Host controller accepts keyboard and (optionally) controller input
pub fn host_controller_bundle(id: usize) -> impl Bundle {
    (
        PlayerController::new(id),
        HostKeyboardController,
        GamepadDevice::None,
        actions!(PlayerController[
            (
                Action::<Movement>::new(),
                DeadZone::default(),
                DeltaScale::default(),
                Scale::splat(300.0),
                Bindings::spawn((
                    Cardinal::wasd_keys(),
                    Axial::left_stick(),
                )),
            ),
            (
                Action::<Jump>::new(),
                bindings![
                    KeyCode::Space,
                    GamepadButton::South,
                ],
            ),
            (
                Action::<Aim>::new(),
                DeadZone::default(),
                Bindings::spawn(Axial::right_stick()),
            ),
            (
                Action::<Shoot>::new(),
                bindings![MouseButton::Left]
            ),
        ]),
    )
}

/// Player controller with gamepad input (not host)
pub fn gamepad_controller_bundle(id: usize, gamepad: Entity) -> impl Bundle {
    (
        PlayerController::new(id),
        GamepadDevice::Single(gamepad),
        actions!(PlayerController[
            (
                Action::<Movement>::new(),
                DeadZone::default(),
                DeltaScale::default(),
                Scale::splat(300.0),
                Bindings::spawn(Axial::left_stick()),
            ),
            (
                Action::<Jump>::new(),
                bindings![GamepadButton::South],
            ),
            (
                Action::<Aim>::new(),
                DeadZone::default(),
                Bindings::spawn(Axial::right_stick()),
            ),
            (
                Action::<Shoot>::new(),
                bindings![GamepadButton::RightTrigger]
            ),
        ]),
    )
}
