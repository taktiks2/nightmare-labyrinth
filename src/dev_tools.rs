//! 開発用ツールシステム
//!
//! 開発時のみ有効なデバッグツールとインスペクター
//! `dev`フィーチャーが有効な場合のみコンパイルされる

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

/// 開発ツールプラグイン
///
/// 開発時に使用するデバッグツールを追加する
/// - World Inspector: ゲーム世界の状態をリアルタイムで確認可能
pub(super) fn plugin(_app: &mut App) {
    // 一時的にWorldInspectorPluginを無効化（プラグイン順序問題のため）
    // TODO: EguiPluginとの順序問題を解決後に再有効化
    
    // app.add_plugins((
    //     bevy_inspector_egui::DefaultInspectorConfigPlugin,
    //     WorldInspectorPlugin::default(),
    // ));
}
