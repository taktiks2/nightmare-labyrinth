use bevy::prelude::*;

use crate::components;
use crate::game::utils;
use crate::resources;

pub fn spawn_board(
    mut commands: Commands,
    game_assets: Res<resources::GameAssets>,
    levels: Res<Assets<resources::Level>>,
) {
    if let Some(level) = levels.get(&game_assets.level) {
        for (y, row) in level.board.iter().enumerate() {
            for (x, id) in row.iter().enumerate() {
                utils::spawn_sprite_at(
                    &mut commands,
                    &game_assets,
                    resources::Tile::Inside,
                    IVec2::new(x as i32, y as i32),
                    Some(0.),
                    Some(format!("inside_{}_{}", x, y)),
                );
                match id {
                    1 => {
                        let entity = utils::spawn_sprite_at(
                            &mut commands,
                            &game_assets,
                            resources::Tile::Block,
                            IVec2::new(x as i32, y as i32),
                            Some(1.),
                            Some("block".to_string()),
                        );
                        commands
                            .entity(entity)
                            .insert((components::Block, components::Savable));
                    }
                    2 => {
                        let entity = utils::spawn_sprite_at(
                            &mut commands,
                            &game_assets,
                            resources::Tile::Goal,
                            IVec2::new(x as i32, y as i32),
                            Some(1.),
                            Some("goal".to_string()),
                        );
                        commands
                            .entity(entity)
                            .insert((components::Goal, components::Savable));
                    }
                    5 => {
                        let entity = utils::spawn_sprite_at(
                            &mut commands,
                            &game_assets,
                            resources::Tile::Coin,
                            IVec2::new(x as i32, y as i32),
                            Some(1.),
                            Some("item".to_string()),
                        );
                        commands.entity(entity).insert((
                            components::Item {
                                name: "coin".to_string(),
                                item_type: components::ItemType::Coin,
                            },
                            components::Savable,
                            Name::new("item"),
                        ));
                    }
                    _ => {}
                }
            }
        }
        for (y, row) in level.actor_board.iter().enumerate() {
            for (x, id) in row.iter().enumerate() {
                match id {
                    3 => {
                        let entity = utils::spawn_sprite_at(
                            &mut commands,
                            &game_assets,
                            resources::Tile::Player,
                            IVec2::new(x as i32, y as i32),
                            Some(2.),
                            Some("player".to_string()),
                        );
                        commands
                            .entity(entity)
                            .insert((components::Player::default(), components::Savable));
                    }
                    4 => {
                        let entity = utils::spawn_sprite_at(
                            &mut commands,
                            &game_assets,
                            resources::Tile::Snake,
                            IVec2::new(x as i32, y as i32),
                            Some(2.),
                            Some("enemy".to_string()),
                        );
                        commands
                            .entity(entity)
                            .insert((components::Enemy, components::Savable));
                    }
                    _ => {}
                }
            }
        }
    }
}
