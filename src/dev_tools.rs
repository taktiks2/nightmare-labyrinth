//! 開発用ツールシステム
//!
//! 開発時のみ有効なデバッグツールとインスペクター
//! `dev`フィーチャーが有効な場合のみコンパイルされる

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

/// 開発ツールプラグイン
///
/// 開発時に使用するデバッグツールを追加する
/// - World Inspector: ゲーム世界の状態をリアルタイムで確認可能
pub(super) fn plugin(app: &mut App) {
    // EguiPluginを先に追加してからWorldInspectorPluginを追加
    // この順序が重要：EguiPlugin → WorldInspectorPlugin
    app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::default()));
}
