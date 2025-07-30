use bevy::{ecs::system::SystemId, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::game::{actions, components};

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "fonts/NotoSansJP-Regular.ttf")]
    pub font_regular: Handle<Font>,
    #[asset(path = "fonts/NotoSansJP-Bold.ttf")]
    pub font_bold: Handle<Font>,
    #[asset(path = "nightmare-labyrinth.aseprite")]
    pub aseprite: Handle<Aseprite>,
    #[asset(key = "level")] // ビルド時ではなく、実行時にアセットをロードする
    pub level: Handle<Level>,
}

#[derive(Deserialize, Serialize, Asset, TypePath)]
pub struct Level {
    pub board: Vec<Vec<i32>>,
    pub actor_board: Vec<Vec<i32>>,
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

#[derive(Resource)]
pub struct QueueSystems {
    pub collect_actor_queue: SystemId,
    pub handle_actor_queue: SystemId,
}

#[derive(Resource, Default)]
pub struct ActionQueue(pub VecDeque<Box<dyn actions::Action>>);

#[derive(Resource, Default)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Resource, Default)]
pub struct DebugConfig {
    pub initial_state: Option<crate::game::states::GameState>,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Inventory>()
        .init_resource::<QueueSystems>()
        .init_resource::<ActionQueue>()
        .init_resource::<ActorQueue>()
        .init_resource::<DebugConfig>();
}
