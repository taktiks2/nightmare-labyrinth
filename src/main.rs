//! Nightmare Labyrinth - ホラー風ローグライクゲーム
//!
//! 悪夢から抜け出せない少女を救出するゲーム
//! - ランダム生成マップ
//! - ターン制システム
//! - 主人公は非戦闘
//! - 恐怖パラメータ（満腹度の代替）

use bevy::prelude::*;

use nightmare_labyrinth::AppPlugin;

/// メインエントリーポイント
///
/// Bevyアプリケーションを初期化し、メインプラグインを追加してゲームを開始
fn main() {
    App::new().add_plugins(AppPlugin).run();
}
