//! Nightmare Labyrinth - コアライブラリ
//!
//! Bevyエンジンを使用したホラー風ローグライクゲームのメインエントリーポイント
//! モジュール構成とプラグイン管理を行う

use bevy::prelude::*;

// モジュール定義
mod camera; // カメラ制御
mod default; // 基本設定
mod dev_tools; // 開発用ツール
pub mod game; // ゲームメインロジック（公開）
mod globals; // グローバル定数
mod mesh; // メッシュ管理

/// メインアプリケーションプラグイン
///
/// すべてのサブシステムを結合し、ゲーム全体を初期化する
/// モジュール式の設計でプラグイン構造を採用
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // コアプラグインを追加（基本設定、カメラ、メッシュ、ゲーム）
        app.add_plugins((default::plugin, camera::plugin, mesh::plugin, game::plugin));

        // 開発ビルド時のみ開発ツールを有効化（最後に追加してプラグイン順序を確保）
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}
