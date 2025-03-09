use bevy::prelude::*;
use std::ops::Neg;

use crate::globals;

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_camera);
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Name::new("game_camera"),
        Transform::from_translation(Vec3::new(
            globals::WINDOW_WIDTH / 2. - globals::SPRITE_SIZE / 2.,
            (globals::WINDOW_HEIGHT / 2. - globals::SPRITE_SIZE / 2.).neg(),
            0.,
        )), // NOTE: 左上から座標が始まるようにnegにする
    ));
}
