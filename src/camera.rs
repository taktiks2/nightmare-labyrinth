//! ゲームカメラ制御システム
//!
//! 2Dゲーム用のカメラ設定と初期化を行う
//! 左上原点の座標系で画面を表示するように設定

use bevy::prelude::*;
use std::ops::Neg;

use crate::globals;

/// メインゲームカメラコンポーネント
///
/// ゲームのメインカメラを識別するためのマーカーコンポーネント
/// Camera2dコンポーネントを自動的に追加する
#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

/// カメラシステムプラグイン
///
/// ゲームカメラの初期化システムを登録
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_camera);
}

/// ゲームカメラを初期化
///
/// ゲームのメインカメラをスポーンし、適切な位置に配置する
/// 左上を原点(0,0)とする座標系で表示するため、Y軸を反転した位置に配置
fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Name::new("game_camera"),
        Transform::from_translation(Vec3::new(
            // X座標: 画面中央からスプライトサイズの半分を引いた位置
            globals::WINDOW_WIDTH / 2. - globals::SPRITE_SIZE / 2.,
            // Y座標: 左上原点にするためにマイナス値を使用
            (globals::WINDOW_HEIGHT / 2. - globals::SPRITE_SIZE / 2.).neg(),
            0.,
        )),
    ));
}
