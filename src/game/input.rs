use bevy::prelude::*;

use crate::game::events;

pub fn handle_keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut input_events: EventWriter<events::InputEvent>,
    mut save_events: EventWriter<events::SaveEvent>,
) {
    if input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyS) {
        save_events.write(events::SaveEvent);
    } else if input.just_released(KeyCode::KeyW) {
        input_events.write(events::InputEvent(IVec2::NEG_Y));
    // NOTE: Ctrl + 保存をしたときに下に移動しないようにする
    } else if !input.pressed(KeyCode::ControlLeft) && input.just_released(KeyCode::KeyS) {
        input_events.write(events::InputEvent(IVec2::Y));
    } else if input.just_released(KeyCode::KeyD) {
        input_events.write(events::InputEvent(IVec2::X));
    } else if input.just_released(KeyCode::KeyA) {
        input_events.write(events::InputEvent(IVec2::NEG_X));
    }
}
