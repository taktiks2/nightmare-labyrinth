use bevy::prelude::*;

#[derive(Event)]
pub enum GameEvent {
    Move(Entity, IVec2),
    Attack(Entity, IVec2),
    Collect(Entity),
}

#[derive(Event)]
pub struct InputEvent(pub IVec2);

#[derive(Event)]
pub struct GameTick;

#[derive(Event)]
pub struct SaveEvent;

#[derive(Event)]
pub struct LoadSaveEvent;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<GameEvent>()
        .add_event::<InputEvent>()
        .add_event::<SaveEvent>()
        .add_event::<LoadSaveEvent>()
        .add_event::<GameTick>();
}
