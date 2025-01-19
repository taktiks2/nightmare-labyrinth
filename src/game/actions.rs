use bevy::prelude::*;

use crate::components;
use crate::events;
use crate::game::utils;

pub fn get_enemy_action(entity: Entity, world: &mut World) -> Option<Box<dyn Action>> {
    let position = world.get::<components::Position>(entity)?.0;
    for dir in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
        if let Some(action) = get_action_at(entity, position + dir, world) {
            return Some(action);
        }
    }
    None
}

pub fn get_action_at(entity: Entity, target: IVec2, world: &mut World) -> Option<Box<dyn Action>> {
    let actions: Vec<Box<dyn Action>> = vec![Box::new(MoveAction { entity, target })];
    for action in actions {
        if action.is_valid(world) {
            return Some(action);
        }
    }
    None
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>>;
    fn is_valid(&self, _world: &mut World) -> bool {
        true
    }
}

pub struct MoveAction {
    pub entity: Entity,
    pub target: IVec2,
}

impl Action for MoveAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        world.get_mut::<components::Position>(self.entity)?.0 = self.target;
        world.send_event::<events::GameEvent>(events::GameEvent::Move(self.entity, self.target));
        None
    }
    fn is_valid(&self, _world: &mut World) -> bool {
        utils::is_on_board(self.target)
    }
}

pub struct AttackAction {
    pub _entity: Entity,
    pub _target: Entity,
}

impl Action for AttackAction {
    fn execute(&self, _world: &mut World) -> Option<Box<dyn Action>> {
        None
    }
    fn is_valid(&self, _world: &mut World) -> bool {
        // NOTE: 動いても良いかどうかの判定
        true
    }
}
