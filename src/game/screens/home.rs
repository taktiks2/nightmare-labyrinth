//! メインゲーム画面システム
//!
//! 実際のゲームプレイが行われるメイン画面を管理
//! ゲームボードの描画、UI表示、プレイヤー入力、ターン制処理を担当
//! ローグライクゲームのコアシステムが集約されたモジュール

use bevy::{color::palettes::css::*, prelude::*};

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
            (setup_board, setup_menu),
        )
        // Playing状態中の継続的な処理（メインループ）
        .add_systems(
            Update,
            (
                save::save,                     // セーブシステム
                input::handle_keyboard_input,   // キーボード入力処理
                turn_base::handle_input_events, // 入力イベント処理
                turn_base::handle_game_events,  // ゲームイベント処理
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
        let level = utils::deserialize_json::<resources::Level>("assets/save.json").unwrap();
        spawn_board(&mut commands, &level, &game_assets);
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
                // コンテンツを中央に縦並びで配置
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(GRAY.into()),
        ))
        .with_children(|parent| {
            // プレースホルダー要素（将来的にインベントリ表示などを追加予定）
            parent.spawn((
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(80.),
                    height: Val::Px(60.),
                    ..default()
                },
                BorderRadius::px(5., 5., 5., 5.),
                BackgroundColor(BLACK.into()),
            ));
        });

    // ボトムステータスバー（画面下部のプレイヤーステータス表示）
    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            // 攻撃力表示（未実装、現在は固定テキスト）
            parent.spawn((
                Text::new("Attack: ".to_string()),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(WHITE.into()),
            ));
            // 防御力表示（未実装、現在は固定テキスト）
            parent.spawn((
                Text::new("Defense: ".to_string()),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(WHITE.into()),
            ));
            // ゴールド表示（未実装、現在は固定テキスト）
            parent.spawn((
                Text::new("Gold: ".to_string()),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(WHITE.into()),
            ));
        });
}
