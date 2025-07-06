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
pub(super) fn plugin(app: &mut App) {
    // ECSインスペクタープラグインを追加
    // Entity、Component、ResourceなどをGUIで確認できる
    app.add_plugins(WorldInspectorPlugin::default());
}
