use bevy::prelude::*;

use crate::game::{components, events, resources, utils};

pub fn save(
    mut save_events: EventReader<events::SaveEvent>,
    level_assets: Res<Assets<resources::Level>>,
    game_assets: Res<resources::GameAssets>,
    query: Query<
        (
            &components::Position,
            Option<&components::Block>,
            Option<&components::Goal>,
            Option<&components::Player>,
            Option<&components::Enemy>,
            Option<&components::Item>,
        ),
        With<components::Savable>,
    >,
) {
    for _event in save_events.read() {
        if let Some(level) = level_assets.get(&game_assets.level) {
            let height = level.board.len();
            let width = level.board[0].len();
            let mut current_board = vec![vec![0; width]; height];
            let mut current_actor_board = vec![vec![0; width]; height];

            for (position, block, goal, player, enemy, item) in query.iter() {
                match (block, goal, player, enemy, item) {
                    (Some(_), _, _, _, _) => {
                        current_board[position.0.y as usize][position.0.x as usize] = 1
                    }
                    (_, Some(_), _, _, _) => {
                        current_board[position.0.y as usize][position.0.x as usize] = 2
                    }
                    (_, _, Some(_), _, _) => {
                        current_actor_board[position.0.y as usize][position.0.x as usize] = 3
                    }
                    (_, _, _, Some(_), _) => {
                        current_actor_board[position.0.y as usize][position.0.x as usize] = 4
                    }
                    (_, _, _, _, Some(_)) => {
                        current_board[position.0.y as usize][position.0.x as usize] = 5
                    }
                    _ => {}
                };
            }

            let level = resources::Level {
                board: current_board,
                actor_board: current_actor_board,
            };

            let _ = utils::serialize_json(&level, "assets/save.json");
        } else {
            error!("Current level not found!");
        }
    }
}
