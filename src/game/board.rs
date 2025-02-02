use bevy::prelude::*;

use crate::game::utils;
use crate::globals;
use crate::resources;

pub fn spawn_board(mut commands: Commands, game_assets: Res<resources::GameAssets>) {
    for y in 0..globals::BOARD_HEIGHT {
        for x in 0..globals::BOARD_WIDTH {
            utils::spawn_sprite_at(
                &mut commands,
                &game_assets,
                get_tile(y as i32, x as i32),
                IVec2::new(x as i32, y as i32),
                Some(0.),
                Some("board".to_string()),
            );
        }
    }
}

fn get_tile(y: i32, x: i32) -> resources::Tile {
    match (y, x) {
        (0, 0) => resources::Tile::LeftTopCorner,
        (y, x) if y == 0 && x == globals::BOARD_WIDTH - 1 => resources::Tile::RightTopCorner,
        (y, x) if y == globals::BOARD_HEIGHT - 1 && x == 0 => resources::Tile::LeftBottomCorner,
        (y, x) if y == globals::BOARD_HEIGHT - 1 && x == globals::BOARD_WIDTH - 1 => {
            resources::Tile::RightBottomCorner
        }
        (0, _) => resources::Tile::TopEdge,
        (y, _) if y == globals::BOARD_HEIGHT - 1 => resources::Tile::BottomEdge,
        (_, 0) => resources::Tile::LeftEdge,
        (_, x) if x == globals::BOARD_WIDTH - 1 => resources::Tile::RightEdge,
        _ => resources::Tile::Center,
    }
}
