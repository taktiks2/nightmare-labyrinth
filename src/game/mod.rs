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
        )
        .init_resource::<QueueSystems>()
        .init_resource::<ActionQueue>()
        .init_resource::<ActorQueue>();
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

#[derive(Resource, Default)]
struct ActionQueue(VecDeque<Box<dyn actions::Action>>);

#[derive(Resource, Default)]
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
    // NOTE: アクター列の先頭を取得
    let Some(&entity) = world.resource::<ActorQueue>().0.front() else {
        // NOTE: なければ新たにアクター列を作る
        let _ = world.run_system(world.resource::<QueueSystems>().collect_actor_queue);
        return;
    };

    // NOTE: player用の処理
    if let Some(mut player) = world.get_mut::<components::Player>(entity) {
        if let Some(target) = player.0.take() {
            if let Some(action) = actions::get_action_at(entity, target, world) {
                world.resource_mut::<ActionQueue>().0.push_back(action);
                world.resource_mut::<ActorQueue>().0.pop_front();
            }
        }
    }

    // NOTE: enemy用の処理
}
