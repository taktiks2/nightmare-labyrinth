use bevy::{ecs::system::SystemId, prelude::*};
use std::collections::VecDeque;
use std::ops::Neg;

mod actions;
mod board;
mod input;
mod objects;
mod utils;

use crate::components;
use crate::events;
use crate::globals;
use crate::resources;
use crate::states;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(states::GameState::Playing),
            (setup_game_camera, board::spawn_board, objects::spawn_object),
        )
        .add_systems(
            Update,
            (
                input::handle_keyboard_input,
                handle_input_events,
                handle_game_events,
            )
                .run_if(in_state(states::GameState::Playing)),
        )
        .add_systems(
            Update,
            handle_action_queue.run_if(on_event::<events::GameTick>),
        )
        .init_resource::<resources::Inventory>()
        .init_resource::<QueueSystems>()
        .init_resource::<ActionQueue>()
        .init_resource::<ActorQueue>();
    }
}

fn setup_game_camera(
    mut commands: Commands,
    game_assets: Res<resources::GameAssets>,
    levels: Res<Assets<resources::Level>>,
) {
    if let Some(level) = levels.get(&game_assets.level) {
        commands.spawn((
            Camera2d,
            Name::new("game_camera"),
            StateScoped(states::GameState::Loading), // NOTE: stateが変わるとワールドから削除できる
            Transform::from_translation(Vec3::new(
                calculate_offset(level.board[0].len() as f32),
                calculate_offset(level.board.len() as f32).neg(),
                0.,
            )), // NOTE: 左上から座標が始まるようにnegにする
        ));
    }
}

fn calculate_offset(size: f32) -> f32 {
    0.5 * globals::SPRITE_SCALE * globals::SPRITE_SIZE * (size - 1.)
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
    let Some(&entity) = world.resource::<ActorQueue>().0.front() else {
        let _ = world.run_system(world.resource::<QueueSystems>().collect_actor_queue);
        return;
    };

    if let Some(mut player) = world.get_mut::<components::Player>(entity) {
        if let Some(target) = player.0.take() {
            if let Some(action) = actions::get_action_at(entity, target, world) {
                world.resource_mut::<ActionQueue>().0.push_back(action);
                world.resource_mut::<ActorQueue>().0.pop_front();
            }
        }
        return;
    }

    world.resource_mut::<ActorQueue>().0.pop_front();
    if let Some(action) = actions::get_enemy_action(entity, world) {
        world.resource_mut::<ActionQueue>().0.push_back(action);
    }
}

fn handle_action_queue(world: &mut World) {
    if let Some(action) = world.resource_mut::<ActionQueue>().0.pop_front() {
        if action.is_valid(world) {
            let result = action.execute(world);
            if let Some(result) = result {
                world.resource_mut::<ActionQueue>().0.push_back(result);
            }
        }
    } else {
        let _ = world.run_system(world.resource::<QueueSystems>().handle_actor_queue);
    }
}

fn handle_input_events(
    mut events: EventReader<events::InputEvent>,
    mut query: Query<(&mut components::Player, &components::Position)>,
) {
    for event in events.read() {
        if let Ok((mut player, position)) = query.get_single_mut() {
            player.0 = Some(position.0 + event.0);
        }
    }
}

fn handle_game_events(
    mut commands: Commands,
    mut game_events: EventReader<events::GameEvent>,
    mut actors: Query<&mut Transform>,
    mut tick_events: EventWriter<events::GameTick>,
    mut inventory: ResMut<resources::Inventory>,
    items: Query<&components::Item>,
) {
    for event in game_events.read() {
        match event {
            events::GameEvent::Move(entity, target) => {
                if let Ok(mut transform) = actors.get_mut(*entity) {
                    transform.translation =
                        utils::position_to_translation(*target, Some(transform.translation.z))
                }
            }
            events::GameEvent::Attack(entity, target) => {}
            events::GameEvent::Collect(entity) => {
                if let Ok(item) = items.get(*entity) {
                    inventory.items.push(item.clone());
                }
                commands.entity(*entity).despawn_recursive();
                debug!("{:?}", inventory);
            }
        }
    }
    tick_events.send(events::GameTick);
}
