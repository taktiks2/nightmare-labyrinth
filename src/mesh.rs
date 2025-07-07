use bevy::prelude::*;

/// メッシュピッキングプラグインの設定
///
/// メッシュオブジェクトに対するクリック検知機能を有効化する
/// UI要素や3Dオブジェクトとのインタラクションを可能にする
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin);
}
