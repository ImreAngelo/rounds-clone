use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use super::components::*;
use super::systems::{Jump, Move};

/// Host controller accepts keyboard and (optionally) controller input
pub fn host_controller_bundle(id: usize) -> impl Bundle {
    (
        PlayerController::new(id),
        HostKeyboardController,
        GamepadDevice::None,
        actions!(PlayerController[
            (
                Action::<Move>::new(),
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
                Action::<Move>::new(),
                DeadZone::default(),
                DeltaScale::default(),
                Scale::splat(300.0),
                Bindings::spawn(Axial::left_stick()),
            ),
            (
                Action::<Jump>::new(),
                bindings![GamepadButton::South],
            ),
        ]),
    )
}
