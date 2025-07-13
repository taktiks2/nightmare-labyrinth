//! ゲームメインロジックモジュール
//!
//! ホラー風ローグライクゲームのコアシステムを管理
//! - ゲーム状態管理
//! - アセットロード
//! - コンポーネントシステム
//! - イベントシステム
//! - ターン制システム

use bevy::prelude::*;

// ゲームシステムモジュール
mod actions; // アクションシステム
mod assets; // アセット管理
mod components; // ECSコンポーネント
mod error_display; // エラー表示システム
mod errors; // エラー型定義
mod events; // ゲームイベント
mod input; // 入力処理
mod resources; // リソース管理
mod save; // セーブシステム
mod screens; // ゲーム画面
mod states; // ゲーム状態
mod turn_base; // ターン制システム
mod utils; // ユーティリティ関数

/// ゲームメインプラグイン
///
/// ゲームのコアシステムを統合し、必要なプラグインをアプリケーションに追加
pub(super) fn plugin(app: &mut App) {
    // ゲームシステムを初期化し、必要なプラグインを追加
    app.add_plugins((
        assets::plugin,        // アセットローダー
        error_display::plugin, // エラー表示システム
        events::plugin,        // イベントシステム
        states::plugin,        // ゲーム状態管理
        resources::plugin,     // リソース管理
        screens::plugin,       // ゲーム画面
    ));
}
