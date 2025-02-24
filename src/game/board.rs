use bevy::prelude::*;

use crate::game::utils;
use crate::resources;

pub fn spawn_board(
    mut commands: Commands,
    game_assets: Res<resources::GameAssets>,
    levels: Res<Assets<resources::Level>>,
) {
    if let Some(level) = levels.get(&game_assets.level) {
        for (y, row) in level.board.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                utils::spawn_sprite_at(
                    &mut commands,
                    &game_assets,
                    get_tile(v),
                    IVec2::new(x as i32, y as i32),
                    Some(0.),
                    Some(format!("board_{}_{}", x, y)),
                );
            }
        }
    }
}

fn get_tile(num: &i32) -> resources::Tile {
    if *num == 1 {
        resources::Tile::Block
    } else {
        resources::Tile::Inside
    }
}
