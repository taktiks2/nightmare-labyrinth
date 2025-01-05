use bevy::prelude::*;

use crate::components;
use crate::game::utils;
use crate::resources;

pub fn spawn_item(mut commands: Commands, atlas: Res<resources::Atlas>) {
    let entity = utils::spawn_sprite_at(
        &mut commands,
        &atlas,
        utils::Texture::Coin,
        IVec2::new(4, 3),
        Some(1.),
    );
    commands.entity(entity).insert(components::Item);
}
