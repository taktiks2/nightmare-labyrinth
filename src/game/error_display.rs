//! エラー表示システム
//!
//! ゲーム内で発生したエラーをユーザーフレンドリーに表示する
//! Bevy 0.16のUIシステムを使用したエラーメッセージ表示

use bevy::{color::palettes::css::*, prelude::*};

use crate::game::{events, states};
use anyhow::Error;

/// エラー表示コンポーネント
///
/// エラーメッセージUIの識別用マーカー
#[derive(Component)]
pub struct ErrorDisplayMarker;

/// エラー表示タイマー
///
/// エラーメッセージの表示時間を管理
#[derive(Component)]
pub struct ErrorDisplayTimer {
    pub timer: Timer,
}

/// エラー表示システムプラグイン
///
/// エラー表示イベントの監視とUI管理を行う
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (handle_error_display_events, update_error_display_timers)
            .run_if(in_state(states::GameState::Playing)),
    );
}

/// エラー表示イベントを処理
///
/// ErrorDisplayEventを監視し、画面上にエラーメッセージを表示
fn handle_error_display_events(
    mut commands: Commands,
    mut error_events: EventReader<events::ErrorDisplayEvent>,
) {
    for event in error_events.read() {
        spawn_error_message(&mut commands, &event.error, event.duration);
    }
}

/// エラーメッセージUIを生成
///
/// 画面上部に一時的にエラーメッセージを表示するUIを作成
fn spawn_error_message(commands: &mut Commands, error: &Error, duration: f32) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.),
            left: Val::Percent(50.),
            width: Val::Px(600.),
            height: Val::Px(80.),
            margin: UiRect::left(Val::Px(-300.)), // 中央寄せのためのマージン
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BackgroundColor(RED.into()),
        BorderColor(DARK_RED.into()),
        BorderRadius::px(8., 8., 8., 8.),
        ErrorDisplayMarker,
        ErrorDisplayTimer {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        },
        StateScoped(states::GameState::Playing),
        children![(
            Text::new(format!("エラー: {}", error)),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(WHITE.into()),
        )],
    ));
}

/// エラー表示タイマーを更新
///
/// 表示時間が経過したエラーメッセージを削除
fn update_error_display_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ErrorDisplayTimer), With<ErrorDisplayMarker>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
