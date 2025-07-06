//! ゲーム状態管理システム
//!
//! Bevyの状態管理システムを使用したゲーム状態の管理
//! ゲームの流れとシーン遷移を制御

use bevy::prelude::*;

/// ゲームの主要状態
///
/// ゲームの異なるシーンやモードを表す状態
/// 状態に応じて異なるシステムやUIが動作する
#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    /// ローディング状態（デフォルト）
    /// アセットのロードや初期化を行う
    #[default]
    Loading,
    /// タイトル画面状態
    /// ゲームのメインメニューを表示
    Title,
    /// ゲームプレイ状態
    /// 実際のゲームプレイが行われる状態
    Playing,
    /// メニュー状態（未実装）
    /// ゲーム内メニューや設定画面
    Menu,
}

/// ゲーム状態プラグイン
///
/// ゲーム状態管理システムを初期化し、状態スコープエンティティを有効化
pub(super) fn plugin(app: &mut App) {
    app
        // 状態スコープエンティティを有効化（状態変化時にエンティティを自動削除）
        .enable_state_scoped_entities::<GameState>()
        // ゲーム状態を初期化（デフォルトはLoading状態）
        .init_state::<GameState>();
}
