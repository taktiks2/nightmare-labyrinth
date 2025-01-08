use bevy::prelude::*;

#[derive(Component)]
pub struct Player(pub Option<IVec2>);

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Attack(pub u32);

#[derive(Component)]
pub struct Defense(pub u32);

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Position(pub IVec2);
