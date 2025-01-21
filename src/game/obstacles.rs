use bevy::prelude::*;

use crate::game::utils;
use crate::resources;

pub fn spawn_obstacle(mut commands: Commands, atlas: Res<resources::Atlas>) {
    utils::spawn_sprite_at(
        &mut commands,
        &atlas,
        utils::Texture::Column,
        IVec2::new(6, 7),
        Some(1.),
        Some("obstacle".to_string()),
    );
    utils::spawn_sprite_at(
        &mut commands,
        &atlas,
        utils::Texture::Column,
        IVec2::new(6, 6),
        Some(1.),
        Some("obstacle".to_string()),
    );
    utils::spawn_sprite_at(
        &mut commands,
        &atlas,
        utils::Texture::Column,
        IVec2::new(6, 5),
        Some(1.),
        Some("obstacle".to_string()),
    );
    utils::spawn_sprite_at(
        &mut commands,
        &atlas,
        utils::Texture::Column,
        IVec2::new(6, 4),
        Some(1.),
        Some("obstacle".to_string()),
    );
}
