use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Player(pub Option<IVec2>);

#[derive(Component, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
}

#[derive(Clone, Debug)]
pub enum ItemType {
    Potion,
    Sword,
    Coin,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Goal;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct Attack(pub u32);

#[derive(Component)]
pub struct Defense(pub u32);

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Position(pub IVec2);
