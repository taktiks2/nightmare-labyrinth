use bevy::prelude::*;

use crate::components;
use crate::game::utils;
use crate::resources;

pub fn spawn_object(
    mut commands: Commands,
    board_layer: Res<resources::GameBoardLayers>,
    atlas: Res<resources::Atlas>,
) {
    for (y, row) in board_layer.object_layer.iter().enumerate() {
        for (x, texture) in row.iter().enumerate() {
            if let Some(texture) = texture {
                let entity = utils::spawn_sprite_at(
                    &mut commands,
                    &atlas,
                    texture.clone(),
                    IVec2::new(x as i32, y as i32),
                    Some(1.),
                    Some("object".to_string()),
                );
                commands
                    .entity(entity)
                    .insert(components::Player::default());
            }
        }
    }
}
