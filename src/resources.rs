use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "nightmare-labyrinth.aseprite")]
    pub aseprite: Handle<Aseprite>,
}

#[derive(Clone, PartialEq)]
pub enum Tile {
    LeftTopCorner,
    RightTopCorner,
    LeftBottomCorner,
    RightBottomCorner,
    TopEdge,
    BottomEdge,
    LeftEdge,
    RightEdge,
    Center,
    Coin,
    Snake,
    Cat,
    Column,
    Heart,
    Sword,
    Shield,
}

impl Tile {
    pub fn as_string(&self) -> String {
        match self {
            Tile::LeftTopCorner => "left-top-corner".into(),
            Tile::RightTopCorner => "right-top-corner".into(),
            Tile::LeftBottomCorner => "left-bottom-corner".into(),
            Tile::RightBottomCorner => "right-bottom-corner".into(),
            Tile::TopEdge => "top-edge".into(),
            Tile::BottomEdge => "bottom-edge".into(),
            Tile::LeftEdge => "left-edge".into(),
            Tile::RightEdge => "right-edge".into(),
            Tile::Center => "center".into(),
            Tile::Coin => "coin".into(),
            Tile::Snake => "snake".into(),
            Tile::Cat => "takcat".into(),
            Tile::Column => "column".into(),
            Tile::Heart => "heart".into(),
            Tile::Sword => "sword".into(),
            Tile::Shield => "shield".into(),
        }
    }
}
