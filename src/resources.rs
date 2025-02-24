use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use serde::{Deserialize, Serialize};

use crate::components;

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "nightmare-labyrinth.aseprite")]
    pub aseprite: Handle<Aseprite>,
    #[asset(path = "level.json")]
    pub level: Handle<Level>,
}

#[derive(Deserialize, Serialize, Asset, TypePath)]
pub struct Level {
    pub board: Vec<Vec<i32>>,
}

#[derive(Clone, PartialEq)]
pub enum Tile {
    Block,
    Outside,
    Inside,
    Sword,
    Shield,
    Goal,
    Coin,
    Heart,
    Snake,
    Player,
}

impl Tile {
    pub fn as_string(&self) -> String {
        match self {
            Tile::Block => "block".into(),
            Tile::Outside => "outside".into(),
            Tile::Inside => "inside".into(),
            Tile::Sword => "sword".into(),
            Tile::Shield => "shield".into(),
            Tile::Goal => "goal".into(),
            Tile::Coin => "coin".into(),
            Tile::Heart => "heart".into(),
            Tile::Snake => "snake".into(),
            Tile::Player => "player".into(),
        }
    }
}

#[derive(Resource, Debug)]
pub struct Inventory {
    pub items: Vec<components::Item>,
}

impl FromWorld for Inventory {
    fn from_world(_world: &mut World) -> Self {
        Self { items: vec![] }
    }
}
