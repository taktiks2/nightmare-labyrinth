use bevy::prelude::*;
use std::ops::Neg;

use crate::components;
use crate::globals;
use crate::resources;

pub enum Texture {
    LeftTopCorner = 0,
    RightTopCorner = 1,
    LeftBottomCorner = 2,
    RightBottomCorner = 3,
    TopEdge = 4,
    BottomEdge = 5,
    LeftEdge = 6,
    RightEdge = 7,
    Center = 8,
    Coin = 9,
    Snake = 10,
    Cat = 11,
}

pub fn spawn_sprite_at(
    commands: &mut Commands,
    atlas: &resources::Atlas,
    texture: Texture,
    position: IVec2,
    z: Option<f32>,
) -> Entity {
    commands
        .spawn((
            Sprite::from_atlas_image(
                atlas.texture.clone(),
                TextureAtlas {
                    layout: atlas.layout.clone(),
                    index: texture as usize,
                },
            ),
            Transform::from_translation(position_to_translation(position, z))
                .with_scale(Vec3::new(globals::SPRITE_SCALE, globals::SPRITE_SCALE, 1.)),
            components::Position(IVec2::new(position.x, position.y)),
        ))
        .id()
}

pub fn position_to_translation(position: IVec2, z: Option<f32>) -> Vec3 {
    Vec3::new(
        position.x as f32 * globals::SPRITE_SIZE * globals::SPRITE_SCALE,
        position.y.neg() as f32 * globals::SPRITE_SIZE * globals::SPRITE_SCALE,
        z.unwrap_or_default(),
    )
}

pub fn is_on_board(position: IVec2) -> bool {
    position.x >= 1
        && position.y >= 1
        && position.x < globals::BOARD_WIDTH - 1
        && position.y < globals::BOARD_HEIGHT - 1
}
