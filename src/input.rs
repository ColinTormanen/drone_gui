use bevy::prelude::*;

use crate::{
    app::{CommandQueue, ControllerState},
    protocol,
};

const THROTTLE_SENSITIVITY: f32 = 0.005;
const MIN_THROTTLE: f32 = 0.3;
const MAX_THROTTLE: f32 = 0.36;
const MAX_TILT_ANGLE: f32 = 8.0_f32.to_radians();

/// Controller input system that reads gamepad axes and updates controller state
/// Left stick: pitch (Y) and yaw (X)
/// Right stick: throttle adjustment (Y) and roll (X)
pub fn controller_input_system(
    time: Res<Time>,
    gamepads: Query<&Gamepad>,
    mut controller_state: ResMut<ControllerState>,
    command_queue: Res<CommandQueue>,
) {
    // Get the first connected gamepad
    let Some(gamepad) = gamepads.iter().next() else {
        return;
    };

    // Left stick Y-axis: pitch (inverted so up is positive)
    if let Some(value) = gamepad.get(GamepadAxis::LeftStickY) {
        controller_state.pitch = -value * MAX_TILT_ANGLE; // Invert Y axis and scale to max angle
    }

    // Left stick X-axis: yaw
    if let Some(value) = gamepad.get(GamepadAxis::LeftStickX) {
        controller_state.roll = value * MAX_TILT_ANGLE;
    }

    // Right trigger: increase base_throttle
    if gamepad.pressed(GamepadButton::RightTrigger2) {
        let adjustment = time.delta_secs() * THROTTLE_SENSITIVITY;
        controller_state.base_throttle =
            (controller_state.base_throttle + adjustment).clamp(MIN_THROTTLE, MAX_THROTTLE);
    }

    // Left trigger: decrease base_throttle
    if gamepad.pressed(GamepadButton::LeftTrigger2) {
        let adjustment = time.delta_secs() * -THROTTLE_SENSITIVITY;
        controller_state.base_throttle =
            (controller_state.base_throttle + adjustment).clamp(MIN_THROTTLE, MAX_THROTTLE);
    }

    // Right stick Y-axis: apply offset to base_throttle (positive Y adds 0.25)
    let throttle_offset = if let Some(value) = gamepad.get(GamepadAxis::RightStickY) {
        value * 0.25
    } else {
        0.0
    };

    // Final throttle = base_throttle + joystick offset
    controller_state.throttle =
        (controller_state.base_throttle + throttle_offset).clamp(MIN_THROTTLE, MAX_THROTTLE);

    // Right stick X-axis: roll
    if let Some(value) = gamepad.get(GamepadAxis::RightStickX) {
        controller_state.yaw = value;
    }

    // A button (South): Start command
    if gamepad.just_pressed(GamepadButton::South)
        && let Err(e) = protocol::send_command_start(&command_queue, 2)
    {
        eprintln!("Failed to send start command: {e}");
    }

    // B button (East): Set base_throttle to 0
    if gamepad.just_pressed(GamepadButton::East) {
        controller_state.base_throttle = 0.0;
    }

    // X button (North): Stop command
    if gamepad.just_pressed(GamepadButton::North)
        && let Err(e) = protocol::send_command_stop(&command_queue, 2)
    {
        eprintln!("Failed to send stop command: {e}");
    }

    // Start button: Emergency stop
    if gamepad.pressed(GamepadButton::Start)
        && let Err(e) = protocol::send_command_emergency_stop(&command_queue, 2)
    {
        eprintln!("EMERGENCY FAILED RUN: {e}");
    }
}
