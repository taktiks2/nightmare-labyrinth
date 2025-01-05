use bevy::prelude::*;

use crate::components;
use crate::game::utils;
use crate::resources;

pub fn spawn_enemy(mut commands: Commands, atlas: Res<resources::Atlas>) {
    let entity = utils::spawn_sprite_at(
        &mut commands,
        &atlas,
        utils::Texture::Snake,
        IVec2::new(5, 5),
        Some(2.),
    );
    commands.entity(entity).insert(components::Enemy);
}
