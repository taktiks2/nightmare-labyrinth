use bevy::prelude::*;

use crate::components;
use crate::events;
use crate::game::utils;
use crate::resources;

pub fn setup_board(
    mut commands: Commands,
    game_assets: Res<resources::GameAssets>,
    levels: Res<Assets<resources::Level>>,
    mut load_save_events: EventReader<events::LoadSaveEvent>,
) {
    let is_load_save = !load_save_events.is_empty();
    load_save_events.clear();

    if is_load_save {
        let level = utils::deserialize_json::<resources::Level>("assets/save.json").unwrap();
        spawn_board(&mut commands, &level, &game_assets);
    } else if let Some(level) = levels.get(&game_assets.level) {
        spawn_board(&mut commands, &level, &game_assets);
    }
}

fn spawn_board(
    mut commands: &mut Commands,
    level: &resources::Level,
    game_assets: &Res<resources::GameAssets>,
) {
    for (y, row) in level.board.iter().enumerate() {
        for (x, id) in row.iter().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);

            // 背景タイルの生成
            utils::spawn_sprite_at(
                &mut commands,
                &game_assets,
                resources::Tile::Inside,
                pos,
                Some(0.),
                Some(format!("inside_{}_{}", x, y)),
            );

            // 静的オブジェクトの生成
            spawn_static_tile(&mut commands, &game_assets, pos, id);
        }
    }
    for (y, row) in level.actor_board.iter().enumerate() {
        for (x, id) in row.iter().enumerate() {
            spawn_actor(
                &mut commands,
                &game_assets,
                IVec2::new(x as i32, y as i32),
                id,
            );
        }
    }
}

fn spawn_static_tile(
    commands: &mut Commands,
    game_assets: &Res<resources::GameAssets>,
    pos: IVec2,
    id: &i32,
) {
    match id {
        1 => {
            let entity = utils::spawn_sprite_at(
                commands,
                game_assets,
                resources::Tile::Block,
                pos,
                Some(1.),
                Some("block".to_string()),
            );
            commands
                .entity(entity)
                .insert((components::Block, components::Savable));
        }
        2 => {
            let entity = utils::spawn_sprite_at(
                commands,
                game_assets,
                resources::Tile::Goal,
                pos,
                Some(1.),
                Some("goal".to_string()),
            );
            commands
                .entity(entity)
                .insert((components::Goal, components::Savable));
        }
        5 => {
            let entity = utils::spawn_sprite_at(
                commands,
                game_assets,
                resources::Tile::Coin,
                pos,
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

fn spawn_actor(
    commands: &mut Commands,
    game_assets: &Res<resources::GameAssets>,
    pos: IVec2,
    id: &i32,
) {
    match id {
        3 => {
            let entity = utils::spawn_sprite_at(
                commands,
                game_assets,
                resources::Tile::Player,
                pos,
                Some(2.),
                Some("player".to_string()),
            );
            commands
                .entity(entity)
                .insert((components::Player::default(), components::Savable));
        }
        4 => {
            let entity = utils::spawn_sprite_at(
                commands,
                game_assets,
                resources::Tile::Snake,
                pos,
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
