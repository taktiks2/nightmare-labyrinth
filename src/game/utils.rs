use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{BufReader, Write};
use std::ops::Neg;

use crate::{
    game::{components, resources, states},
    globals,
};

pub fn spawn_sprite_at(
    commands: &mut Commands,
    game_assets: &Res<resources::GameAssets>,
    tile: resources::Tile,
    position: IVec2,
    z: Option<f32>,
    name: Option<String>,
) -> Entity {
    let mut entity_commands =
        commands.spawn((
            AseSpriteSlice {
                name: tile.as_string(),
                aseprite: game_assets.aseprite.clone(),
            },
            Transform::from_translation(position_to_translation(position, z))
                .with_scale(Vec3::new(globals::SPRITE_SCALE, globals::SPRITE_SCALE, 1.)),
            StateScoped(states::GameState::Playing), // NOTE: stateが変わるとワールドから削除できる
            components::Position(IVec2::new(position.x, position.y)),
        ));
    if let Some(name) = name {
        entity_commands.insert(Name::new(name));
    }
    entity_commands.id()
}

pub fn position_to_translation(position: IVec2, z: Option<f32>) -> Vec3 {
    Vec3::new(
        position.x as f32 * globals::SPRITE_SIZE * globals::SPRITE_SCALE,
        position.y.neg() as f32 * globals::SPRITE_SIZE * globals::SPRITE_SCALE,
        z.unwrap_or_default(),
    )
}

// NOTE: JSON -> Rust構造体
pub fn deserialize_json<T>(path: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let target: T = serde_json::from_reader(reader)?;
    Ok(target)
}

// NOTE: Rust構造体 -> JSON
pub fn serialize_json<T>(data: &T, path: &str) -> Result<(), serde_json::Error>
where
    T: Serialize,
{
    let json_string = serde_json::to_string_pretty(data)?;
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");
    Ok(())
}
