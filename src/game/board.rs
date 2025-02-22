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
    let is_border =
        y == 0 || y == globals::BOARD_HEIGHT - 1 || x == 0 || x == globals::BOARD_WIDTH - 1;

    if is_border {
        resources::Tile::Block
    } else {
        resources::Tile::Inside
    }
}
