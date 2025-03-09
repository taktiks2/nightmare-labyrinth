use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::game::{resources, states};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(JsonAssetPlugin::<resources::Level>::new(&["json"])) // NOTE: jsonファイルを読み込むのに必要 & 引数には拡張子部分を指定
        .add_plugins(AsepriteUltraPlugin)
        .add_loading_state(
            // NOTE: アセットがロードされるまでローディングを出す
            LoadingState::new(states::GameState::Loading)
                .continue_to_state(states::GameState::Title)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "dynamic_asset.assets.ron",
                )
                .load_collection::<resources::GameAssets>(),
        );
}
