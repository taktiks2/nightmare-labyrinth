use bevy::prelude::*;

use crate::game::utils;
use crate::resources;

pub fn spawn_board(
    mut commands: Commands,
    board_layer: Res<resources::GameBoardLayers>,
    atlas: Res<resources::Atlas>,
) {
    for (y, row) in board_layer.base_layer.iter().enumerate() {
        for (x, texture) in row.iter().enumerate() {
            utils::spawn_sprite_at(
                &mut commands,
                &atlas,
                texture.clone(),
                IVec2::new(x as i32, y as i32),
                Some(0.),
                Some("board".to_string()),
            );
        }
    }
}
