use bevy::prelude::*;

use crate::game::{actions, components, events, resources, utils};

/// ターン制ゲームシステムの初期化
///
/// アクターキューの収集と処理を行うシステムを登録する
impl FromWorld for resources::QueueSystems {
    fn from_world(world: &mut World) -> Self {
        Self {
            collect_actor_queue: world.register_system(collect_actor_queue),
            handle_actor_queue: world.register_system(handle_actor_queue),
        }
    }
}

/// アクターの行動順序を決定してキューに登録
///
/// プレイヤーを最優先に設定し、その後敵キャラクターを追加する
/// ターン制システムの基本となる行動順序を管理する
pub fn collect_actor_queue(
    enemy_query: Query<Entity, (With<components::Enemy>, Without<components::Player>)>,
    player_query: Query<Entity, With<components::Player>>,
    mut queue: ResMut<resources::ActorQueue>,
) {
    // 敵キャラクターをキューに登録
    queue.0 = enemy_query.iter().collect();

    // プレイヤーを最優先（キューの先頭）に配置
    if let Ok(player) = player_query.single() {
        queue.0.push_front(player);
    }
}

/// アクターキューから次の行動者を処理
///
/// キューの先頭からアクターを取得し、適切な行動を決定する
/// プレイヤーと敵で異なる行動処理ロジックを適用
pub fn handle_actor_queue(world: &mut World) {
    // キューが空の場合は新しくアクターを収集
    let Some(&entity) = world.resource::<resources::ActorQueue>().0.front() else {
        let _ = world.run_system(
            world
                .resource::<resources::QueueSystems>()
                .collect_actor_queue,
        );
        return;
    };

    // プレイヤーの行動処理
    if let Some(mut player) = world.get_mut::<components::Player>(entity) {
        if let Some(target) = player.0.take() {
            if let Some(action) = actions::get_action_at(entity, target, world) {
                world
                    .resource_mut::<resources::ActionQueue>()
                    .0
                    .push_back(action);
                world.resource_mut::<resources::ActorQueue>().0.pop_front();
            }
        }
        return;
    }

    // 敵の行動処理
    world.resource_mut::<resources::ActorQueue>().0.pop_front();
    if let Some(action) = actions::get_enemy_action(entity, world) {
        world
            .resource_mut::<resources::ActionQueue>()
            .0
            .push_back(action);
    }
}

/// アクションキューの処理を実行
///
/// キューからアクションを取得し、有効性を確認してから実行する
/// 実行結果として新しいアクションが生成される場合はキューに追加
pub fn handle_action_queue(world: &mut World) {
    if let Some(action) = world.resource_mut::<resources::ActionQueue>().0.pop_front() {
        // アクションの有効性を確認して実行
        if action.is_valid(world) {
            let result = action.execute(world);
            // 実行結果として新しいアクションが生成された場合はキューに追加
            if let Some(result) = result {
                world
                    .resource_mut::<resources::ActionQueue>()
                    .0
                    .push_back(result);
            }
        }
    } else {
        // アクションキューが空の場合は次のアクターを処理
        let _ = world.run_system(
            world
                .resource::<resources::QueueSystems>()
                .handle_actor_queue,
        );
    }
}

/// プレイヤーの入力イベントを処理
///
/// 入力された方向をプレイヤーの移動目標として設定
/// ターン制における主人公の行動決定を行う
pub fn handle_input_events(
    mut events: EventReader<events::InputEvent>,
    mut query: Query<(&mut components::Player, &components::Position)>,
) {
    for event in events.read() {
        if let Ok((mut player, position)) = query.single_mut() {
            // 現在位置に入力方向を加えて移動目標を設定
            player.0 = Some(position.0 + event.0);
        }
    }
}

/// ゲームイベントの処理とゲーム状態の更新
///
/// 移動、攻撃、アイテム収集などのゲームイベントを処理し
/// 各イベント処理後にゲームティックを発行してターンを進行
pub fn handle_game_events(
    mut commands: Commands,
    mut game_events: EventReader<events::GameEvent>,
    mut actors: Query<&mut Transform>,
    mut tick_events: EventWriter<events::GameTick>,
    mut inventory: ResMut<resources::Inventory>,
    items: Query<&components::Item>,
) {
    for event in game_events.read() {
        match event {
            // エンティティの移動処理
            events::GameEvent::Move(entity, target) => {
                if let Ok(mut transform) = actors.get_mut(*entity) {
                    transform.translation =
                        utils::position_to_translation(*target, Some(transform.translation.z))
                }
            }
            // 攻撃処理（現在は未実装）
            events::GameEvent::Attack(_entity, _target) => {}
            // アイテム収集処理
            events::GameEvent::Collect(entity) => {
                if let Ok(item) = items.get(*entity) {
                    inventory.items.push(item.clone());
                }
                commands.entity(*entity).despawn();
                debug!("{:?}", inventory);
            }
        }
    }
    // 全イベント処理後にゲームティックを発行
    tick_events.write(events::GameTick);
}
