use bevy::prelude::*;
use bevy::{asset::AssetMetaCheck, log::LogPlugin};

use crate::globals;

/// Bevyの基本プラグイン設定を行う
///
/// アプリケーションの基盤となるプラグインの設定を行い、
/// ウィンドウ、アセット、画像、ログの各システムを初期化する
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(configure_window_plugin())
            .set(configure_asset_plugin())
            .set(configure_image_plugin())
            .set(configure_log_plugin()),
    );
}

/// ウィンドウプラグインの設定
///
/// ゲームウィンドウのタイトル、解像度、リサイズ可否を設定
fn configure_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Nightmare Labyrinth".to_string(),
            resolution: [globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT].into(),
            resizable: false, // ウィンドウサイズの変更を不可にする
            ..default()
        }),
        ..default()
    }
}

/// アセットプラグインの設定
///
/// WebAssemblyビルドでのアセットファイル読み込みに必要な設定
fn configure_asset_plugin() -> AssetPlugin {
    AssetPlugin {
        meta_check: AssetMetaCheck::Never, // wasmビルドで必要
        ..default()
    }
}

/// 画像プラグインの設定
///
/// ピクセルアートが適切に表示されるよう、最近傍補間を使用
fn configure_image_plugin() -> ImagePlugin {
    ImagePlugin::default_nearest()
}

/// ログプラグインの設定
///
/// デバッグ用ログの出力設定を行い、アプリケーション固有のログのみを表示
fn configure_log_plugin() -> LogPlugin {
    LogPlugin {
        filter: "nightmare_labyrinth".to_string(), // 自身のログのみ出力
        ..default()
    }
}
