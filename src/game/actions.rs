use bevy::prelude::*;

use crate::components;
use crate::events;
use crate::game::utils;

pub fn handle_game_events(
    mut commands: Commands,
    mut events: EventReader<events::GameEvent>,
    mut query: Query<(Entity, &mut Transform), With<components::Player>>,
) {
    for event in events.read() {
        match event {
            events::GameEvent::Move(entity, target) => {
                println!("Move: {:?} -> {:?}", entity, target);
            }
            events::GameEvent::Attack(entity, target) => {
                println!("Attack: {:?} -> {:?}", entity, target);
            }
        }
    }
}

pub fn handle_player_move(
    mut input_events: EventReader<events::InputEvent>,
    mut query: Query<(&mut components::Position, &mut Transform), With<components::Player>>,
) {
    for event in input_events.read() {
        if let Ok((mut position, mut transform)) = query.get_single_mut() {
            position.0 += event.0;
            transform.translation =
                utils::position_to_translation(position.0, Some(transform.translation.z));
        }
    }
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>>;
    fn is_valid(&self, _world: &mut World) -> bool {
        true
    }
}
