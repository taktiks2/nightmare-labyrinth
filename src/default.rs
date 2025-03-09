use bevy::prelude::*;
use bevy::{asset::AssetMetaCheck, log::LogPlugin};

use crate::globals;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Nightmare Labyrinth".to_string(),
                    resolution: [globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT].into(), // NOTE: Windowサイズの指定
                    resizable: false, // NOTE: Windowサイズの変更を不可にする
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                // NOTE: wasmビルドでassetファイルを読み込むのに必要
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(ImagePlugin::default_nearest()) // NOTE: 画像がぼやけないようにする
            .set(LogPlugin {
                // NOTE: bevy標準のログを出力するのに必要
                filter: "nightmare_labyrinth".to_string(), // NOTE: 自身の出したログのみ出力するさせる
                ..default()
            }),
    );
}
