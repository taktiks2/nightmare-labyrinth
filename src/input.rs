use bevy::prelude::*;

use crate::events;

pub fn handle_keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut input_events: EventWriter<events::InputEvent>,
) {
    if input.just_released(KeyCode::KeyW) {
        input_events.send(events::InputEvent(IVec2::NEG_Y));
    }
    if input.just_released(KeyCode::KeyS) {
        input_events.send(events::InputEvent(IVec2::Y));
    }
    if input.just_released(KeyCode::KeyD) {
        input_events.send(events::InputEvent(IVec2::X));
    }
    if input.just_released(KeyCode::KeyA) {
        input_events.send(events::InputEvent(IVec2::NEG_X));
    }
}
