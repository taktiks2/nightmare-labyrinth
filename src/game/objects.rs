use bevy::prelude::*;

use crate::components;
use crate::game::utils;
use crate::resources;

pub fn spawn_object(mut commands: Commands, game_assets: Res<resources::GameAssets>) {
    let entity = utils::spawn_sprite_at(
        &mut commands,
        &game_assets,
        resources::Tile::Player,
        IVec2::new(4, 4),
        Some(1.),
        Some("player".to_string()),
    );
    add_components_for_texture(&mut commands, entity, &resources::Tile::Player);

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
        resources::Tile::Block,
        IVec2::new(6, 6),
        Some(1.),
        Some("obstacle".to_string()),
    );
    add_components_for_texture(&mut commands, entity, &resources::Tile::Block);

    let entity = utils::spawn_sprite_at(
        &mut commands,
        &game_assets,
        resources::Tile::Block,
        IVec2::new(6, 7),
        Some(1.),
        Some("obstacle".to_string()),
    );
    add_components_for_texture(&mut commands, entity, &resources::Tile::Block);

    let entity = utils::spawn_sprite_at(
        &mut commands,
        &game_assets,
        resources::Tile::Coin,
        IVec2::new(3, 3),
        Some(1.),
        Some("item".to_string()),
    );
    add_components_for_texture(&mut commands, entity, &resources::Tile::Coin);

    let entity = utils::spawn_sprite_at(
        &mut commands,
        &game_assets,
        resources::Tile::Coin,
        IVec2::new(3, 3),
        Some(1.),
        Some("item".to_string()),
    );
    add_components_for_texture(&mut commands, entity, &resources::Tile::Coin);
}

fn add_components_for_texture(commands: &mut Commands, entity: Entity, tile: &resources::Tile) {
    match tile {
        resources::Tile::Player => {
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
            commands.entity(entity).insert((
                components::Item {
                    name: "coin".to_string(),
                    item_type: components::ItemType::Coin,
                },
                Name::new("item"),
            ));
        }
        resources::Tile::Block => {
            commands
                .entity(entity)
                .insert((components::Obstacle, Name::new("obstacle")));
        }
        _ => {} // NOTE: その他のテクスチャの場合は何もしない
    }
}
