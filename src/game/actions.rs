//! ゲームアクションシステム
//!
//! ゲーム内のアクション（移動、攻撃など）を抽象化したシステム
//! コマンドパターンを使用して、アクションの実行とバリデーションを行う

use bevy::prelude::*;

use crate::game::{components, events, resources};

/// 敵キャラクターのアクションを取得
///
/// 敵が取るべきアクションを決定する
/// 現在は単純な移動のみを実装（四方向をチェック）
///
/// # 引数
/// * `entity` - 敵のEntity
/// * `world` - ゲーム世界の参照
///
/// # 戻り値
/// 実行可能なアクション、またはNone
pub fn get_enemy_action(entity: Entity, world: &mut World) -> Option<Box<dyn Action>> {
    let position = world.get::<components::Position>(entity)?.0;

    // 四方向をチェックして移動可能な方向を探す
    for dir in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
        if let Some(action) = get_action_at(entity, position + dir, world) {
            return Some(action);
        }
    }
    None
}

/// 指定した位置で実行可能なアクションを取得
///
/// 現在は移動アクションのみをサポート
/// 将来的には攻撃やその他のアクションを追加予定
///
/// # 引数
/// * `entity` - アクションを実行するEntity
/// * `target` - ターゲット位置
/// * `world` - ゲーム世界の参照
///
/// # 戻り値
/// 実行可能なアクション、またはNone
pub fn get_action_at(entity: Entity, target: IVec2, world: &mut World) -> Option<Box<dyn Action>> {
    // 利用可能なアクションのリストを作成（現在は移動のみ）
    let actions: Vec<Box<dyn Action>> = vec![Box::new(MoveAction { entity, target })];

    // 最初に有効なアクションを返す
    for action in actions {
        if action.is_valid(world) {
            return Some(action);
        }
    }
    None
}

/// ゲームアクションの基本トレイト
///
/// すべてのゲームアクションが実装すべきインターフェース
/// コマンドパターンを使用して、アクションの実行とバリデーションを分離
pub trait Action: Send + Sync {
    /// アクションを実行する
    ///
    /// # 引数
    /// * `world` - ゲーム世界の可変参照
    ///
    /// # 戻り値
    /// 後続アクションがある場合はそのアクション、なければNone
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>>;

    /// アクションが実行可能かどうかをチェック
    ///
    /// # 引数
    /// * `world` - ゲーム世界の可変参照
    ///
    /// # 戻り値
    /// 実行可能ならtrue、そうでなければfalse
    fn is_valid(&self, _world: &mut World) -> bool {
        true
    }
}

/// 移動アクション
///
/// エンティティを指定した位置に移動させるアクション
/// 移動先にアイテムがある場合は自動的に収集する
pub struct MoveAction {
    /// 移動するエンティティ
    pub entity: Entity,
    /// 移動先の位置
    pub target: IVec2,
}

impl Action for MoveAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        // 移動先にアイテムがあるかチェックし、あれば収集イベントを発生
        if let Some((item_entity, _)) = world
            .query_filtered::<(Entity, &components::Position), With<components::Item>>()
            .iter(world)
            .find(|(_, position)| position.0 == self.target)
        {
            world.send_event::<events::GameEvent>(events::GameEvent::Collect(item_entity));
        }

        // エンティティの位置を更新
        if let Some(mut position) = world.get_mut::<components::Position>(self.entity) {
            position.0 = self.target;
        } else {
            error!(
                "エンティティ {:?} の Position コンポーネントが見つかりません",
                self.entity
            );
            return None;
        }

        // 移動イベントを発生
        world.send_event::<events::GameEvent>(events::GameEvent::Move(self.entity, self.target));

        None
    }

    fn is_valid(&self, world: &mut World) -> bool {
        // レベルデータとアセットを取得して移動可能性をチェック
        if let (Some(game_assets), Some(level_assets)) = (
            world.get_resource::<resources::GameAssets>(),
            world.get_resource::<Assets<resources::Level>>(),
        ) {
            if let Some(level) = level_assets.get(&game_assets.level) {
                let value = level.board[self.target.y as usize][self.target.x as usize];
                // 値が1の場合は障害物（ブロック）なので移動不可
                if value == 1 {
                    return false;
                }
            }
        }
        true
    }
}

/// 攻撃アクション（未実装）
///
/// 敵やオブジェクトを攻撃するアクション
/// 現在はプレースホルダー実装
pub struct AttackAction {
    /// 攻撃者のEntity
    pub _entity: Entity,
    /// 攻撃対象のEntity
    pub _target: Entity,
}

impl Action for AttackAction {
    fn execute(&self, _world: &mut World) -> Option<Box<dyn Action>> {
        // TODO: 攻撃処理の実装
        None
    }

    fn is_valid(&self, _world: &mut World) -> bool {
        // TODO: 攻撃可能性のチェックを実装
        true
    }
}
