use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use std::ops::Neg;

use crate::components;
use crate::globals;
use crate::resources;
use crate::states;

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

pub fn is_on_board(position: IVec2) -> bool {
    position.x >= 1
        && position.y >= 1
        && position.x < globals::BOARD_WIDTH - 1
        && position.y < globals::BOARD_HEIGHT - 1
}
