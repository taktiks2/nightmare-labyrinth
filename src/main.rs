use bevy::prelude::*;
use std::ops::Neg;

mod components;
mod events;
mod game;
mod globals;
mod input;
mod resources;
mod states;

use globals::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(game::GamePlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, input::handle_keyboard_input)
        .add_event::<events::GameEvent>()
        .add_event::<events::InputEvent>()
        .add_event::<events::GameTick>()
        .init_resource::<resources::Atlas>()
        .run();
}

fn calculate_offset(size: f32) -> f32 {
    0.5 * SPRITE_SCALE * SPRITE_SIZE * (size - 1.)
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(
            calculate_offset(BOARD_WIDTH as f32),
            calculate_offset(BOARD_HEIGHT as f32).neg(),
            0.,
        )), // NOTE: 左上から座標が始まるようにnegにする
    ));
}
