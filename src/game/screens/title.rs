//! タイトル画面システム
//!
//! ゲームのメインメニュー画面を管理するシステム
//! ゲームタイトル、新規ゲーム、コンティニューボタンを表示
//! セーブファイルの存在をチェックし、適切なボタンを表示
//! ボタンクリックでゲーム状態を遷移させる

use bevy::{color::palettes::css::*, prelude::*};
use std::path::Path;

use crate::game::{events, resources, states};

/// タイトル画面プラグイン
///
/// Title状態に入った時にタイトル画面を表示するシステムを登録
/// メインメニューの表示とユーザー操作を処理
pub(super) fn plugin(app: &mut App) {
    // Title状態に入った時にタイトル画面をセットアップ
    app.add_systems(OnEnter(states::GameState::Title), setup_title);
}

/// タイトル画面のUIをセットアップ
///
/// ゲームタイトル、メニューボタンを含むメインメニューを作成
/// セーブファイルの存在をチェックし、あればContinueボタンを表示
/// 常にNew Gameボタンを表示し、ゲームの新規開始を可能にする
///
/// # 引数
/// * `commands` - EntityとComponentをスポーンするためのコマンド
/// * `game_assets` - ゲームアセット（フォントなど）へのアクセス
fn setup_title(mut commands: Commands, game_assets: Res<resources::GameAssets>) {
    // セーブファイルの存在をチェック（Continueボタン表示のため）
    let save_file_exists = Path::new("assets/save.json").exists();

    // メインコンテナを作成（縦方向のフレックスレイアウト）
    commands
        .spawn((
            Node {
                // コンテンツを中央に配置し、間隔を空ける
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                // フルスクリーンサイズ
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                // 上下にパディングを追加
                padding: UiRect {
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    top: Val::Px(100.),
                    bottom: Val::Px(100.),
                },
                ..default()
            },
            Name::new("title_screen"),
            // Title状態のみで存在し、状態変化時に自動削除
            StateScoped(states::GameState::Title),
            // グレーの背景色
            BackgroundColor(GRAY.into()),
            children![
                // ゲームタイトルを表示
                (
                    Text::new("悪夢の迷宮"),
                    TextFont {
                        // 太字フォントを使用
                        font: game_assets.font_bold.clone(),
                        font_size: 60.0,
                        ..default()
                    },
                ),
            ],
        ))
        .with_children(|parent| {
            // セーブファイルがある場合はContinueボタンを表示
            if save_file_exists {
                parent
                    .spawn((
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(240.),
                            height: Val::Px(60.),
                            ..default()
                        },
                        // 黒い背景と丸みを帯びたボタン
                        BackgroundColor(BLACK.into()),
                        BorderRadius::px(5., 5., 5., 5.),
                        Button,
                        children![(
                            Text::new("Continue"),
                            TextFont {
                                font_size: 40.0,
                                ..default()
                            },
                        )],
                    ))
                    // Continueボタンのクリックイベントを監視
                    .observe(handle_continue_click);
            }

            // New Gameボタンを常に表示
            parent
                .spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(240.),
                        height: Val::Px(60.),
                        ..default()
                    },
                    // 黒い背景と丸みを帯びたボタン
                    BackgroundColor(BLACK.into()),
                    BorderRadius::px(5., 5., 5., 5.),
                    Button,
                    children![(
                        Text::new("New Game"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                    )],
                ))
                // New Gameボタンのクリックイベントを監視
                .observe(handle_new_game_click);
        });
}

/// Continueボタンのクリックイベントを処理
///
/// セーブデータのロードイベントを発行し、ゲーム状態をPlayingに変更
/// セーブファイルが存在する場合のみ表示されるボタンの処理
///
/// # 引数
/// * `_click` - クリックイベントトリガー（未使用）
/// * `next_state` - ゲーム状態を変更するためのリソース
/// * `load_save_events` - セーブデータロードイベントを発行するためのライター
pub fn handle_continue_click(
    _click: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<states::GameState>>,
    mut load_save_events: EventWriter<events::LoadSaveEvent>,
) {
    // セーブデータのロードを要求するイベントを発行
    load_save_events.write(events::LoadSaveEvent);
    // ゲームプレイ状態に遷移
    next_state.set(states::GameState::Playing);
}

/// New Gameボタンのクリックイベントを処理
///
/// 新規ゲームを開始し、ゲーム状態をPlayingに変更
/// セーブデータのロードは行わず、新しいゲームを開始
///
/// # 引数
/// * `_click` - クリックイベントトリガー（未使用）
/// * `next_state` - ゲーム状態を変更するためのリソース
pub fn handle_new_game_click(
    _click: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<states::GameState>>,
) {
    // ゲームプレイ状態に遷移（新規ゲーム開始）
    next_state.set(states::GameState::Playing);
}
