//! メインゲーム画面システム
//!
//! 実際のゲームプレイが行われるメイン画面を管理
//! ゲームボードの描画、UI表示、プレイヤー入力、ターン制処理を担当
//! ローグライクゲームのコアシステムが集約されたモジュール

use bevy::{color::palettes::css::*, prelude::*, text::LineHeight};

use crate::game::{components, events, input, resources, save, states, turn_base, utils};
use crate::globals;

/// サイドメニューの幅（ピクセル）
///
/// 画面右端に表示される情報パネルの幅
const SIDE_MENU_WIDTH: f32 = 224.;

/// ボトムタブの高さ（ピクセル）
///
/// 画面下部に表示されるステータスバーの高さ
const BOTTOM_TAB_HEIGHT: f32 = 48.;

/// メインゲーム画面プラグイン
///
/// Playing状態時のゲームボード、UI、システムを初期化・管理
/// ゲームのメインロジック（入力、ターン制、セーブ）を結合
pub(super) fn plugin(app: &mut App) {
    app
        // Playing状態に入った時の初期化処理
        .add_systems(
            OnEnter(states::GameState::Playing),
            (setup_board, setup_menu, setup_log_window),
        )
        // Playing状態中の継続的な処理（メインループ）
        .add_systems(
            Update,
            (
                save::save,                     // セーブシステム
                input::handle_keyboard_input,   // キーボード入力処理
                turn_base::handle_input_events, // 入力イベント処理
                turn_base::handle_game_events,  // ゲームイベント処理
                update_log_window_position,     // ログウィンドウの位置更新
                update_log_display,             // ログ表示更新
            )
                .run_if(in_state(states::GameState::Playing)),
        )
        // ゲームティックイベント時のアクション処理
        .add_systems(
            Update,
            turn_base::handle_action_queue.run_if(on_event::<events::GameTick>),
        );
}

/// ゲームボードをセットアップ
///
/// セーブデータのロードイベントをチェックし、適切なレベルデータを読み込んでボードを生成
/// セーブデータがある場合はそれを優先し、なければデフォルトレベルを使用
///
/// # 引数
/// * `commands` - EntityとComponentをスポーンするためのコマンド
/// * `game_assets` - ゲームアセットへのアクセス
/// * `levels` - レベルデータアセットへのアクセス
/// * `load_save_events` - セーブデータロードイベントのリーダー
pub fn setup_board(
    mut commands: Commands,
    game_assets: Res<resources::GameAssets>,
    levels: Res<Assets<resources::Level>>,
    mut load_save_events: EventReader<events::LoadSaveEvent>,
) {
    // セーブデータのロード要求があるかチェック
    let is_load_save = !load_save_events.is_empty();
    load_save_events.clear();

    if is_load_save {
        // セーブファイルからレベルデータを読み込み
        match utils::deserialize_json::<resources::Level>("assets/save.json") {
            Ok(level) => {
                spawn_board(&mut commands, &level, &game_assets);
            }
            Err(e) => {
                error!("セーブファイルの読み込みに失敗しました: {:#}", e);

                // エラー表示イベントは現在のシステムでは簡単に送信できないため、
                // ログ出力で代替（将来的にはイベントシステムの改善が必要）

                // セーブファイルが読み込めない場合はデフォルトレベルを使用
                if let Some(level) = levels.get(&game_assets.level) {
                    spawn_board(&mut commands, &level, &game_assets);
                }
            }
        }
    } else if let Some(level) = levels.get(&game_assets.level) {
        // デフォルトレベルを使用
        spawn_board(&mut commands, &level, &game_assets);
    }
}

/// レベルデータからゲーヤボードを生成
///
/// レベルデータの2つのボード（静的オブジェクトとアクター）を読み取り、
/// 各タイルで適切なスプライトとコンポーネントを持つEntityをスポーン
///
/// # 引数
/// * `commands` - Entityをスポーンするためのコマンド
/// * `level` - レベルデータ（ボード情報を含む）
/// * `game_assets` - ゲームアセット（スプライトなど）
fn spawn_board(
    mut commands: &mut Commands,
    level: &resources::Level,
    game_assets: &Res<resources::GameAssets>,
) {
    // 静的オブジェクトボードを処理（背景、ブロック、アイテムなど）
    for (y, row) in level.board.iter().enumerate() {
        for (x, id) in row.iter().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);

            // すべてのタイルに背景タイルを配置（Z座標: 0）
            utils::spawn_sprite_at(
                &mut commands,
                &game_assets,
                resources::Tile::Inside,
                pos,
                Some(0.),
                Some(format!("inside_{}_{}", x, y)),
            );

            // 静的オブジェクトをスポーン（Z座標: 1）
            spawn_static_tile(&mut commands, &game_assets, pos, id);
        }
    }

    // アクターボードを処理（プレイヤー、敵など）
    for (y, row) in level.actor_board.iter().enumerate() {
        for (x, id) in row.iter().enumerate() {
            spawn_actor(
                &mut commands,
                &game_assets,
                IVec2::new(x as i32, y as i32),
                id,
            );
        }
    }
}

/// 静的タイルをスポーン
///
/// レベルデータのIDに応じて適切な静的オブジェクトをスポーン
/// ブロック、ゴール、アイテムなどの移動しないオブジェクトを処理
///
/// # 引数
/// * `commands` - Entityをスポーンするためのコマンド
/// * `game_assets` - ゲームアセット（スプライトなど）
/// * `pos` - スポーン位置
/// * `id` - レベルデータのID（タイルタイプを決定）
fn spawn_static_tile(
    commands: &mut Commands,
    game_assets: &Res<resources::GameAssets>,
    pos: IVec2,
    id: &i32,
) {
    match id {
        // ID 1: ブロック（通行不可能な障害物）
        1 => {
            let entity = utils::spawn_sprite_at(
                commands,
                game_assets,
                resources::Tile::Block,
                pos,
                Some(1.),
                Some("block".to_string()),
            );
            commands
                .entity(entity)
                .insert((components::Block, components::Savable));
        }
        // ID 2: ゴール（ゲームクリア地点）
        2 => {
            let entity = utils::spawn_sprite_at(
                commands,
                game_assets,
                resources::Tile::Goal,
                pos,
                Some(1.),
                Some("goal".to_string()),
            );
            commands
                .entity(entity)
                .insert((components::Goal, components::Savable));
        }
        // ID 5: コイン（収集アイテム）
        5 => {
            let entity = utils::spawn_sprite_at(
                commands,
                game_assets,
                resources::Tile::Coin,
                pos,
                Some(1.),
                Some("item".to_string()),
            );
            commands.entity(entity).insert((
                components::Item {
                    name: "coin".to_string(),
                    item_type: components::ItemType::Coin,
                },
                components::Savable,
                Name::new("item"),
            ));
        }
        // その他のID: 何もスポーンしない
        _ => {}
    }
}

/// アクター（動的キャラクター）をスポーン
///
/// レベルデータのIDに応じてプレイヤーや敵などの動的キャラクターをスポーン
/// これらのキャラクターはターン制システムで移動やアクションを行う
///
/// # 引数
/// * `commands` - Entityをスポーンするためのコマンド
/// * `game_assets` - ゲームアセット（スプライトなど）
/// * `pos` - スポーン位置
/// * `id` - レベルデータのID（キャラクタータイプを決定）
fn spawn_actor(
    commands: &mut Commands,
    game_assets: &Res<resources::GameAssets>,
    pos: IVec2,
    id: &i32,
) {
    match id {
        // ID 3: プレイヤー（ユーザーが操作するキャラクター）
        3 => {
            let entity = utils::spawn_sprite_at(
                commands,
                game_assets,
                resources::Tile::Player,
                pos,
                Some(2.), // アクターはZ座標 2 で表示（最前面）
                Some("player".to_string()),
            );
            commands
                .entity(entity)
                .insert((components::Player::default(), components::Savable));
        }
        // ID 4: 敵キャラクター（AI制御）
        4 => {
            let entity = utils::spawn_sprite_at(
                commands,
                game_assets,
                resources::Tile::Snake,
                pos,
                Some(2.), // アクターはZ座標 2 で表示（最前面）
                Some("enemy".to_string()),
            );
            commands
                .entity(entity)
                .insert((components::Enemy, components::Savable));
        }
        // その他のID: 何もスポーンしない
        _ => {}
    }
}

/// ゲームUIメニューをセットアップ
///
/// ゲームプレイ中のサイドメニューとボトムステータスバーを作成
/// サイドメニューはインベントリやステータス表示用（未実装）
/// ボトムステータスバーはプレイヤーのステータスを表示
///
/// # 引数
/// * `commands` - EntityとComponentをスポーンするためのコマンド
pub fn setup_menu(mut commands: Commands) {
    // サイドメニュー（画面右端の情報パネル）
    commands
        .spawn((
            Node {
                // 絶対位置で画面右端に固定
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                right: Val::Px(0.),
                width: Val::Px(SIDE_MENU_WIDTH),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(5.)),
                ..default()
            },
            BackgroundColor(DARK_GRAY.into()),
        ))
        .with_children(|parent| {
            // アイテムスロット
            (0..20).for_each(|i| {
                parent.spawn((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Px(30.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(3.)),
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                    children![(
                        Text::new("空スロット"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
                    )],
                ));
            });
        });

    // ボトムステータスバー（画面下部のプレイヤーステータス表示）
    commands.spawn((
        Node {
            // 絶対位置で画面下部に固定（サイドメニューを除いた幅）
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.),
            left: Val::Px(0.),
            width: Val::Px(globals::WINDOW_WIDTH - SIDE_MENU_WIDTH),
            height: Val::Px(BOTTOM_TAB_HEIGHT),
            // コンテンツを中央に均等配置
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        },
        BackgroundColor(GRAY.into()),
        children![
            // 攻撃力表示（未実装、現在は固定テキスト）
            (
                Text::new("Attack: ".to_string()),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(WHITE.into()),
            ),
            // 防御力表示（未実装、現在は固定テキスト）
            (
                Text::new("Defense: ".to_string()),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(WHITE.into()),
            ),
            // ゴールド表示（未実装、現在は固定テキスト）
            (
                Text::new("Gold: ".to_string()),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(WHITE.into()),
            ),
        ],
    ));
}

fn setup_log_window(mut commands: Commands) {
    commands.spawn((
        components::LogContainer,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(SIDE_MENU_WIDTH),
            bottom: Val::Px(BOTTOM_TAB_HEIGHT),
            width: Val::Px(400.),
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(20.)),
            row_gap: Val::Px(5.),
            ..default()
        },
        Name::new("log_container"),
    ));
}

/// 主人公の位置に基づいてログウィンドウの位置を更新
fn update_log_window_position(
    player_query: Query<&Transform, (With<components::Player>, Changed<Transform>)>,
    mut log_container_query: Query<&mut Node, With<components::LogContainer>>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut node) = log_container_query.single_mut() {
            // プレイヤーの画面座標を計算（仮定：32ピクセル/タイル）
            let player_screen_x = player_transform.translation.x;

            let half_board = (globals::WINDOW_WIDTH - SIDE_MENU_WIDTH) / 2.;

            // 画面の中央より右にいる場合は左側に表示、左にいる場合は右側に表示
            if player_screen_x > half_board {
                // プレイヤーが右側にいる場合、ログウィンドウを左側に配置
                node.left = Val::Px(0.);
                node.right = Val::Auto;
            } else {
                // プレイヤーが左側にいる場合、ログウィンドウを右側に配置
                node.left = Val::Auto;
                node.right = Val::Px(SIDE_MENU_WIDTH);
            }
        }
    }
}

/// ログ表示を更新
fn update_log_display(
    mut commands: Commands,
    log_queue: Res<resources::LogQueue>,
    game_assets: Res<resources::GameAssets>,
    log_container_query: Query<Entity, With<components::LogContainer>>,
    log_entry_query: Query<Entity, With<components::LogEntry>>,
) {
    if log_queue.is_changed() {
        let logs = log_queue.get_recent_logs();

        // 既存のログエントリをすべて削除
        for log_entry in log_entry_query.iter() {
            commands.entity(log_entry).despawn();
        }

        if let Ok(log_container_entity) = log_container_query.single() {
            // 各ログメッセージを個別の枠で囲んで表示
            for log_entry in logs.iter() {
                let log_frame = commands
                    .spawn((
                        components::LogEntry,
                        Node {
                            width: Val::Percent(100.),
                            padding: UiRect::all(Val::Px(8.)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(5.)),
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
                        children![(
                            Text::new(log_entry.message.clone()),
                            TextFont {
                                font: game_assets.font_regular.clone(),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(WHITE.into()),
                        )],
                    ))
                    .id();

                commands
                    .entity(log_container_entity)
                    .add_children(&[log_frame]);
            }
        }
    }
}
