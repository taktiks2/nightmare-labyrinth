use bevy::prelude::*;

mod actions;
mod board;
mod enemies;
mod items;
mod player;
mod utils;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                board::spawn_board,
                player::spawn_player,
                enemies::spawn_enemy,
                items::spawn_item,
            ),
        )
        .add_systems(
            Update,
            (actions::handle_player_move, actions::handle_game_events),
        );
    }
}
