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
pub mod resources; // リソース管理（公開）
mod save; // セーブシステム
mod screens; // ゲーム画面
pub mod states; // ゲーム状態（公開）
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

    // デバッグ機能：指定された初期状態への遷移システム
    app.add_systems(Update, debug_state_transition);
}

/// デバッグ用状態遷移システム
///
/// DebugConfigで指定された初期状態がある場合、
/// ローディング完了後（Title状態到達時）に指定状態へ遷移
fn debug_state_transition(
    mut commands: Commands,
    debug_config: Res<resources::DebugConfig>,
    current_state: Res<State<states::GameState>>,
    mut next_state: ResMut<NextState<states::GameState>>,
) {
    // デバッグ状態が指定されており、現在Title状態の場合のみ実行
    if let Some(target_state) = &debug_config.initial_state {
        if *current_state.get() == states::GameState::Title {
            // 指定状態に遷移し、デバッグ設定をクリア
            next_state.set(target_state.clone());
            commands.insert_resource(resources::DebugConfig {
                initial_state: None,
            });
        }
    }
}
