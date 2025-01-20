use bevy::prelude::*;

use crate::game::utils;
use crate::globals;
use crate::resources;

pub fn spawn_board(mut commands: Commands, atlas: Res<resources::Atlas>) {
    for y in 0..globals::BOARD_HEIGHT {
        for x in 0..globals::BOARD_WIDTH {
            utils::spawn_sprite_at(
                &mut commands,
                &atlas,
                board_texture(y, x),
                IVec2::new(x, y),
                Some(0.),
                Some("board".to_string()),
            );
        }
    }
}

fn board_texture(y: i32, x: i32) -> utils::Texture {
    match (y, x) {
        (0, 0) => utils::Texture::LeftTopCorner,
        (y, x) if y == 0 && x == globals::BOARD_WIDTH - 1 => utils::Texture::RightTopCorner,
        (y, x) if y == globals::BOARD_HEIGHT - 1 && x == 0 => utils::Texture::LeftBottomCorner,
        (y, x) if y == globals::BOARD_HEIGHT - 1 && x == globals::BOARD_WIDTH - 1 => {
            utils::Texture::RightBottomCorner
        }
        (0, _) => utils::Texture::TopEdge,
        (y, _) if y == globals::BOARD_HEIGHT - 1 => utils::Texture::BottomEdge,
        (_, 0) => utils::Texture::LeftEdge,
        (_, x) if x == globals::BOARD_WIDTH - 1 => utils::Texture::RightEdge,
        _ => utils::Texture::Center,
    }
}
