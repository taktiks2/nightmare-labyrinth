//! ゲーム入力管理システム
//!
//! プレイヤーのキーボード入力を処理し、ゲームイベントに変換するシステム
//! ローグライクゲームの基本操作（移動、セーブ）をサポート
//!
//! # サポートする操作
//! - **移動**: WASDキーで四方向の移動
//! - **セーブ**: Ctrl+Sでゲームのセーブ
//!
//! # キーマッピング
//! | キー | 動作 | 方向ベクトル |
//! |------|------|------------|
//! | W    | 上移動 | (0, -1)    |
//! | S    | 下移動 | (0, 1)     |
//! | A    | 左移動 | (-1, 0)    |
//! | D    | 右移動 | (1, 0)     |
//! | Ctrl+S | セーブ | -          |

use bevy::prelude::*;

use crate::game::events;

/// キーボード入力を処理し、ゲームイベントに変換
///
/// プレイヤーのキーボード入力を監視し、適切なゲームイベントを発行する
/// 移動入力はInputEvent、セーブ入力はSaveEventとして発行
///
/// # 入力処理の特徴
/// - **ターン制システム**: キーリリース時にアクションを発行（連続入力防止）
/// - **無競合入力**: Ctrl+Sセーブ時に下方向移動を防止
/// - **即座応答**: キー入力は即座にイベントとして発行
///
/// # 引数
/// * `input` - キーボード入力状態へのアクセス
/// * `input_events` - プレイヤー移動イベントのライター
/// * `save_events` - ゲームセーブイベントのライター
pub fn handle_keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut input_events: EventWriter<events::InputEvent>,
    mut save_events: EventWriter<events::SaveEvent>,
) {
    // セーブコマンドの処理（Ctrl + S）
    // 最優先で処理し、他の入力との競合を防ぐ
    if input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyS) {
        save_events.write(events::SaveEvent);
    }
    // 上方向移動（Wキー）
    // ゲーム座標系では上がマイナスY値
    else if input.just_released(KeyCode::KeyW) {
        input_events.write(events::InputEvent(IVec2::NEG_Y));
    }
    // 下方向移動（Sキー）
    // Ctrlが押されていない場合のみ処理（セーブコマンドとの競合防止）
    else if !input.pressed(KeyCode::ControlLeft) && input.just_released(KeyCode::KeyS) {
        input_events.write(events::InputEvent(IVec2::Y));
    }
    // 右方向移動（Dキー）
    // ゲーム座標系では右がプラスX値
    else if input.just_released(KeyCode::KeyD) {
        input_events.write(events::InputEvent(IVec2::X));
    }
    // 左方向移動（Aキー）
    // ゲーム座標系では左がマイナスX値
    else if input.just_released(KeyCode::KeyA) {
        input_events.write(events::InputEvent(IVec2::NEG_X));
    }
}
