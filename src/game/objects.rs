use bevy::prelude::*;

use crate::components;
use crate::game::utils;
use crate::resources;

pub fn spawn_object(
    mut commands: Commands,
    game_assets: Res<resources::GameAssets>,
    levels: Res<Assets<resources::Level>>,
) {
    if let Some(level) = levels.get(&game_assets.level) {
        for (y, row) in level.objects.iter().enumerate() {
            for (x, object_id) in row.iter().enumerate() {
                match object_id {
                    2 => {
                        let entity = utils::spawn_sprite_at(
                            &mut commands,
                            &game_assets,
                            resources::Tile::Goal,
                            IVec2::new(x as i32, y as i32),
                            Some(1.),
                            Some("goal".to_string()),
                        );

                        commands.entity(entity).insert(components::Goal);
                    }
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
                            .insert(components::Player::default());
                    }
                    4 => {
                        let entity = utils::spawn_sprite_at(
                            &mut commands,
                            &game_assets,
                            resources::Tile::Snake,
                            IVec2::new(x as i32, y as i32),
                            Some(1.),
                            Some("enemy".to_string()),
                        );
                        commands.entity(entity).insert(components::Enemy);
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
                            Name::new("item"),
                        ));
                    }
                    _ => {}
                }
            }
        }
    }
}
