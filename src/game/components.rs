//! ゲームECSコンポーネント定義
//!
//! BevyのEntity-Component-Systemアーキテクチャで使用する
//! ゲームオブジェクトのコンポーネントを定義

use bevy::prelude::*;

/// プレイヤーコンポーネント
///
/// プレイヤーの行動を管理するコンポーネント
/// Option<IVec2>で次の移動先を保持（Noneの場合は移動しない）
#[derive(Component, Default, Debug)]
pub struct Player(pub Option<IVec2>);

/// アイテムコンポーネント
///
/// ゲーム内のアイテムを表すコンポーネント
/// アイテム名とタイプを保持
#[derive(Component, Clone, Debug)]
pub struct Item {
    /// アイテム名
    pub name: String,
    /// アイテムの種類
    pub item_type: ItemType,
}

/// アイテムの種類
///
/// ゲーム内で出現するアイテムのタイプを定義
#[derive(Clone, Debug)]
pub enum ItemType {
    /// 回復アイテム（未実装）
    Potion,
    /// 武器（未実装）
    Sword,
    /// コイン（収集アイテム）
    Coin,
}

/// 敵キャラクターコンポーネント
///
/// エンティティが敵であることを示すマーカーコンポーネント
#[derive(Component)]
pub struct Enemy;

/// ゴールコンポーネント
///
/// ゲームのゴール地点を示すマーカーコンポーネント
#[derive(Component)]
pub struct Goal;

/// ブロックコンポーネント
///
/// 通行不可能な障害物を示すマーカーコンポーネント
#[derive(Component)]
pub struct Block;

/// セーブ可能コンポーネント
///
/// セーブデータに含めるエンティティを示すマーカーコンポーネント
#[derive(Component)]
pub struct Savable;

/// 攻撃力コンポーネント
///
/// エンティティの攻撃力を表すコンポーネント
#[derive(Component)]
pub struct Attack(pub u32);

/// 防御力コンポーネント
///
/// エンティティの防御力を表すコンポーネント
#[derive(Component)]
pub struct Defense(pub u32);

/// 体力コンポーネント
///
/// エンティティの体力を表すコンポーネント
#[derive(Component)]
pub struct Health(pub u32);

/// 位置コンポーネント
///
/// エンティティのゲーム世界内の位置を表すコンポーネント
#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct LogWindow;
