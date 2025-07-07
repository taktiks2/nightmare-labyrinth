use bevy::prelude::*;

use crate::game::{components, events, resources, utils};

/// ゲームの現在状態をJSONファイルに保存するシステム
///
/// # 機能
/// - 現在のレベルの状態を取得
/// - エンティティの位置情報から盤面を再構築
/// - 静的オブジェクト（ブロック、ゴール、アイテム）とアクター（プレイヤー、敵）を分離
/// - JSON形式でファイルに保存
pub fn save(
    mut save_events: EventReader<events::SaveEvent>,
    level_assets: Res<Assets<resources::Level>>,
    game_assets: Res<resources::GameAssets>,
    query: Query<
        (
            &components::Position,
            Option<&components::Block>,
            Option<&components::Goal>,
            Option<&components::Player>,
            Option<&components::Enemy>,
            Option<&components::Item>,
        ),
        With<components::Savable>,
    >,
) {
    for _event in save_events.read() {
        if let Some(level) = level_assets.get(&game_assets.level) {
            // 現在のレベルサイズを取得
            let height = level.board.len();
            let width = level.board[0].len();

            // 静的オブジェクト用とアクター用の盤面を初期化
            let mut current_board = create_empty_board(width, height);
            let mut current_actor_board = create_empty_board(width, height);

            // 全エンティティを走査して盤面を構築
            for (position, block, goal, player, enemy, item) in query.iter() {
                let pos = (position.0.y as usize, position.0.x as usize);

                // コンポーネントの種類に応じて適切な盤面に配置
                match (block, goal, player, enemy, item) {
                    (Some(_), _, _, _, _) => set_tile(&mut current_board, pos, TileType::Block),
                    (_, Some(_), _, _, _) => set_tile(&mut current_board, pos, TileType::Goal),
                    (_, _, Some(_), _, _) => {
                        set_tile(&mut current_actor_board, pos, TileType::Player)
                    }
                    (_, _, _, Some(_), _) => {
                        set_tile(&mut current_actor_board, pos, TileType::Enemy)
                    }
                    (_, _, _, _, Some(_)) => set_tile(&mut current_board, pos, TileType::Item),
                    _ => {} // 該当なしの場合は何もしない
                }
            }

            // 新しいレベルデータを作成
            let save_level = resources::Level {
                board: current_board,
                actor_board: current_actor_board,
            };

            // JSONファイルに保存
            match utils::serialize_json(&save_level, "assets/save.json") {
                Ok(_) => info!("ゲームデータを正常に保存しました"),
                Err(e) => error!("保存に失敗しました: {:?}", e),
            }
        } else {
            error!("現在のレベルが見つかりません");
        }
    }
}

/// タイルの種類を定義する列挙型
#[derive(Debug)]
enum TileType {
    Block = 1,
    Goal = 2,
    Player = 3,
    Enemy = 4,
    Item = 5,
}

/// 空の盤面を作成するヘルパー関数
///
/// # 引数
/// * `width` - 盤面の幅
/// * `height` - 盤面の高さ///
/// # 戻り値
/// 指定されたサイズの0で初期化された2次元ベクター
fn create_empty_board(width: usize, height: usize) -> Vec<Vec<i32>> {
    vec![vec![0; width]; height]
}

/// 指定された位置にタイルを設定するヘルパー関数
///
/// # 引数
/// * `board` - 設定対象の盤面
/// * `pos` - 設定位置 (y, x)
/// * `tile_type` - 設定するタイルの種類
fn set_tile(board: &mut [Vec<i32>], pos: (usize, usize), tile_type: TileType) {
    board[pos.0][pos.1] = tile_type as i32;
}
