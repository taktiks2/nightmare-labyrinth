use bevy::{ecs::system::SystemId, prelude::*};
use std::collections::VecDeque;

mod actions;
mod board;
mod enemies;
mod items;
mod player;
mod utils;

use crate::components;

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

#[derive(Resource)]
struct QueueSystems {
    collect_actor_queue: SystemId,
    handle_actor_queue: SystemId,
}

impl FromWorld for QueueSystems {
    fn from_world(world: &mut World) -> Self {
        Self {
            collect_actor_queue: world.register_system(collect_actor_queue),
            handle_actor_queue: world.register_system(handle_actor_queue),
        }
    }
}

#[derive(Resource)]
struct ActionQueue(VecDeque<Box<dyn actions::Action>>);

#[derive(Resource)]
struct ActorQueue(VecDeque<Entity>);

fn collect_actor_queue(
    enemy_query: Query<Entity, (With<components::Enemy>, Without<components::Player>)>,
    player_query: Query<Entity, With<components::Player>>,
    mut queue: ResMut<ActorQueue>,
) {
    queue.0 = enemy_query.iter().collect();
    if let Ok(player) = player_query.get_single() {
        queue.0.push_front(player);
    }
}

fn handle_actor_queue(world: &mut World) {
    let Some(&entity) = world.resource::<ActorQueue>().0.front() else {
        let _ = world.run_system(world.resource::<QueueSystems>().collect_actor_queue);
        return;
    };
}
