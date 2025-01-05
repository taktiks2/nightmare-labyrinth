use bevy::prelude::*;

use crate::globals;

#[derive(Resource)]
pub struct Atlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

impl FromWorld for Atlas {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let texture = asset_server.load("tilemap.png");
        let layout = TextureAtlasLayout::from_grid(
            UVec2::splat(globals::SPRITE_SIZE as u32),
            4,
            3,
            Some(UVec2::splat(1)),
            None,
        );
        let mut texture_atlas_layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let texture_atlas_layouts = texture_atlas_layouts.add(layout);
        Self {
            layout: texture_atlas_layouts,
            texture,
        }
    }
}
