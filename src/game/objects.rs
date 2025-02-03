use bevy::prelude::*;

use crate::components;
use crate::game::utils;
use crate::resources;

pub fn spawn_object(mut commands: Commands, game_assets: Res<resources::GameAssets>) {
    let entity = utils::spawn_sprite_at(
        &mut commands,
        &game_assets,
        resources::Tile::Cat,
        IVec2::new(4, 4),
        Some(1.),
        Some("player".to_string()),
    );
    add_components_for_texture(&mut commands, entity, &resources::Tile::Cat);

    let entity = utils::spawn_sprite_at(
        &mut commands,
        &game_assets,
        resources::Tile::Snake,
        IVec2::new(4, 5),
        Some(1.),
        Some("enemy".to_string()),
    );
    add_components_for_texture(&mut commands, entity, &resources::Tile::Snake);

    let entity = utils::spawn_sprite_at(
        &mut commands,
        &game_assets,
        resources::Tile::Column,
        IVec2::new(6, 6),
        Some(1.),
        Some("obstacle".to_string()),
    );
    add_components_for_texture(&mut commands, entity, &resources::Tile::Column);

    let entity = utils::spawn_sprite_at(
        &mut commands,
        &game_assets,
        resources::Tile::Column,
        IVec2::new(6, 7),
        Some(1.),
        Some("obstacle".to_string()),
    );
    add_components_for_texture(&mut commands, entity, &resources::Tile::Column);
}

fn add_components_for_texture(commands: &mut Commands, entity: Entity, tile: &resources::Tile) {
    match tile {
        resources::Tile::Cat => {
            commands
                .entity(entity)
                .insert((components::Player::default(), Name::new("player")));
        }
        resources::Tile::Snake => {
            commands
                .entity(entity)
                .insert((components::Enemy, Name::new("enemy")));
        }
        resources::Tile::Coin => {
            commands
                .entity(entity)
                .insert((components::Item, Name::new("item")));
        }
        resources::Tile::Column => {
            commands
                .entity(entity)
                .insert((components::Obstacle, Name::new("obstacle")));
        }
        _ => {} // NOTE: その他のテクスチャの場合は何もしない
    }
}
