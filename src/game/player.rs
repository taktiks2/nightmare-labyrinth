use bevy::prelude::*;

use crate::components;
use crate::game::utils;
use crate::resources;

pub fn spawn_player(mut commands: Commands, atlas: Res<resources::Atlas>) {
    let entity = utils::spawn_sprite_at(
        &mut commands,
        &atlas,
        utils::Texture::Cat,
        IVec2::new(3, 3),
        Some(2.),
    );
    commands.entity(entity).insert(components::Player);
}
