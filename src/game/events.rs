//! ゲームイベントシステム
//!
//! Bevyのイベントシステムを使用したゲームイベントの定義
//! システム間の通信とゲームロジックの結合を担当

use bevy::prelude::*;

/// ゲームアクションイベント
///
/// ゲーム内で発生するアクションを表すイベント
/// ターン制システムで使用される
#[derive(Event)]
pub enum GameEvent {
    /// エンティティの移動イベント
    /// (Entity: 移動するエンティティ, IVec2: 移動先の位置)
    Move(Entity, IVec2),
    /// 攻撃イベント（未実装）
    /// (Entity: 攻撃者, IVec2: 攻撃先の位置)
    Attack(Entity, IVec2),
    /// アイテム収集イベント
    /// (Entity: 収集するアイテム)
    Collect(Entity),
}

/// 入力イベント
///
/// プレイヤーの入力を表すイベント
/// 方向ベクトルで移動方向を指定
#[derive(Event)]
pub struct InputEvent(pub IVec2);

/// ゲームティックイベント
///
/// ゲームの1ターンが終了したことを示すイベント
/// ターン制システムで使用
#[derive(Event)]
pub struct GameTick;

/// セーブイベント
///
/// ゲームのセーブを要求するイベント
#[derive(Event)]
pub struct SaveEvent;

/// セーブデータロードイベント
///
/// セーブデータのロードを要求するイベント
#[derive(Event)]
pub struct LoadSaveEvent;

/// イベントシステムプラグイン
///
/// ゲームで使用するすべてのイベントをアプリケーションに登録
pub(super) fn plugin(app: &mut App) {
    app.add_event::<GameEvent>()
        .add_event::<InputEvent>()
        .add_event::<SaveEvent>()
        .add_event::<LoadSaveEvent>()
        .add_event::<GameTick>();
}
