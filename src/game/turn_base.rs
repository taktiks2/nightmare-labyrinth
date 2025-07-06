use bevy::prelude::*;

use crate::game::{actions, components, events, resources, utils};

impl FromWorld for resources::QueueSystems {
    fn from_world(world: &mut World) -> Self {
        Self {
            collect_actor_queue: world.register_system(collect_actor_queue),
            handle_actor_queue: world.register_system(handle_actor_queue),
        }
    }
}

pub fn collect_actor_queue(
    enemy_query: Query<Entity, (With<components::Enemy>, Without<components::Player>)>,
    player_query: Query<Entity, With<components::Player>>,
    mut queue: ResMut<resources::ActorQueue>,
) {
    queue.0 = enemy_query.iter().collect();
    if let Ok(player) = player_query.single() {
        queue.0.push_front(player);
    }
}

pub fn handle_actor_queue(world: &mut World) {
    let Some(&entity) = world.resource::<resources::ActorQueue>().0.front() else {
        let _ = world.run_system(
            world
                .resource::<resources::QueueSystems>()
                .collect_actor_queue,
        );
        return;
    };

    if let Some(mut player) = world.get_mut::<components::Player>(entity) {
        if let Some(target) = player.0.take() {
            if let Some(action) = actions::get_action_at(entity, target, world) {
                world
                    .resource_mut::<resources::ActionQueue>()
                    .0
                    .push_back(action);
                world.resource_mut::<resources::ActorQueue>().0.pop_front();
            }
        }
        return;
    }

    world.resource_mut::<resources::ActorQueue>().0.pop_front();
    if let Some(action) = actions::get_enemy_action(entity, world) {
        world
            .resource_mut::<resources::ActionQueue>()
            .0
            .push_back(action);
    }
}

pub fn handle_action_queue(world: &mut World) {
    if let Some(action) = world.resource_mut::<resources::ActionQueue>().0.pop_front() {
        if action.is_valid(world) {
            let result = action.execute(world);
            if let Some(result) = result {
                world
                    .resource_mut::<resources::ActionQueue>()
                    .0
                    .push_back(result);
            }
        }
    } else {
        let _ = world.run_system(
            world
                .resource::<resources::QueueSystems>()
                .handle_actor_queue,
        );
    }
}

pub fn handle_input_events(
    mut events: EventReader<events::InputEvent>,
    mut query: Query<(&mut components::Player, &components::Position)>,
) {
    for event in events.read() {
        if let Ok((mut player, position)) = query.single_mut() {
            player.0 = Some(position.0 + event.0);
        }
    }
}

pub fn handle_game_events(
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
            events::GameEvent::Attack(_entity, _target) => {}
            events::GameEvent::Collect(entity) => {
                if let Ok(item) = items.get(*entity) {
                    inventory.items.push(item.clone());
                }
                commands.entity(*entity).despawn();
                debug!("{:?}", inventory);
            }
        }
    }
    tick_events.write(events::GameTick);
}
