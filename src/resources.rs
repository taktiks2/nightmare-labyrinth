use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct Atlas {
    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 4,
        rows = 3,
        padding_x = 1,
        padding_y = 1,
    ))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "tilemap.png")]
    pub texture: Handle<Image>,
}

// TODO: eguiをデバッグ時にだけ出せるようにする
