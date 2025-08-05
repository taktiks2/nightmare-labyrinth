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
    mut log_queue: ResMut<resources::LogQueue>,
    players: Query<(), With<components::Player>>,
) {
    for event in game_events.read() {
        match event {
            // エンティティの移動処理
            events::GameEvent::Move(entity, target) => {
                if let Ok(mut transform) = actors.get_mut(*entity) {
                    transform.translation =
                        utils::position_to_translation(*target, Some(transform.translation.z));

                    // プレイヤーの移動の場合、ログに座標を記録
                    if players.get(*entity).is_ok() {
                        log_queue.add_log(
                            format!("移動: ({}, {})", target.x, target.y),
                            resources::LogType::PlayerMove,
                        );
                    }
                }
            }
            // 攻撃処理（現在は未実装）
            events::GameEvent::Attack(_entity, _target) => {}
            // アイテム収集処理
            events::GameEvent::Collect(entity) => {
                if let Ok(item) = items.get(*entity) {
                    inventory.items.push(item.clone());
                    log_queue.add_log(
                        format!("{}を取得しました", item.name),
                        resources::LogType::ItemPickup,
                    );
                }
                commands.entity(*entity).despawn();
            }
        }
    }
    // 全イベント処理後にゲームティックを発行
    tick_events.write(events::GameTick);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_actor_queue_with_player_and_enemies() {
        // プレイヤーと敵がいる状態でキューの収集をテスト
        let mut app = App::new();
        app.init_resource::<resources::ActorQueue>();
        app.add_systems(Update, collect_actor_queue);

        // エンティティをアプリに追加
        let player = app.world_mut().spawn((components::Player(None),)).id();
        let enemy1 = app.world_mut().spawn(components::Enemy).id();
        let enemy2 = app.world_mut().spawn(components::Enemy).id();

        app.update();

        // キューの内容を確認
        let queue = app.world().resource::<resources::ActorQueue>();
        assert_eq!(queue.0.len(), 3);
        assert_eq!(queue.0.front(), Some(&player)); // プレイヤーが先頭
    }

    #[test]
    fn test_collect_actor_queue_player_only() {
        // プレイヤーのみの場合のキュー収集をテスト
        let mut app = App::new();
        app.init_resource::<resources::ActorQueue>();
        app.add_systems(Update, collect_actor_queue);

        let player = app.world_mut().spawn((components::Player(None),)).id();

        app.update();

        let queue = app.world().resource::<resources::ActorQueue>();
        assert_eq!(queue.0.len(), 1);
        assert_eq!(queue.0.front(), Some(&player));
    }

    #[test]
    fn test_collect_actor_queue_enemies_only() {
        // 敵のみの場合のキュー収集をテスト
        let mut app = App::new();
        app.init_resource::<resources::ActorQueue>();
        app.add_systems(Update, collect_actor_queue);

        let enemy1 = app.world_mut().spawn(components::Enemy).id();
        let enemy2 = app.world_mut().spawn(components::Enemy).id();

        app.update();

        let queue = app.world().resource::<resources::ActorQueue>();
        assert_eq!(queue.0.len(), 2);
        // プレイヤーがいない場合、敵のみがキューに入る
        assert!(queue.0.contains(&enemy1));
        assert!(queue.0.contains(&enemy2));
    }

    #[test]
    fn test_handle_input_events() {
        // 入力イベントの処理をテスト
        let mut app = App::new();
        app.add_event::<events::InputEvent>();
        app.add_systems(Update, handle_input_events);

        let player = app
            .world_mut()
            .spawn((
                components::Player(None),
                components::Position(IVec2::new(0, 0)),
            ))
            .id();

        // 入力イベントを送信
        app.world_mut()
            .send_event(events::InputEvent(IVec2::new(1, 0)));

        app.update();

        // プレイヤーの移動目標が設定されたことを確認
        let player_component = app.world().get::<components::Player>(player).unwrap();
        assert_eq!(player_component.0, Some(IVec2::new(1, 0)));
    }

    #[test]
    fn test_handle_game_events_move() {
        // 移動イベントの処理をテスト
        let mut app = App::new();
        app.init_resource::<resources::Inventory>();
        app.add_event::<events::GameEvent>();
        app.add_event::<events::GameTick>();
        app.add_systems(Update, handle_game_events);

        let entity = app
            .world_mut()
            .spawn(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
            .id();

        // 移動イベントを送信
        app.world_mut()
            .send_event(events::GameEvent::Move(entity, IVec2::new(1, 1)));

        app.update();

        // エンティティの位置が更新されたことを確認
        let transform = app.world().get::<Transform>(entity).unwrap();
        let expected_translation = utils::position_to_translation(IVec2::new(1, 1), Some(0.0));
        assert_eq!(transform.translation, expected_translation);
    }

    #[test]
    fn test_handle_game_events_collect() {
        // アイテム収集イベントの処理をテスト
        let mut app = App::new();
        app.init_resource::<resources::Inventory>();
        app.add_event::<events::GameEvent>();
        app.add_event::<events::GameTick>();
        app.add_systems(Update, handle_game_events);

        let item = components::Item {
            name: "テストアイテム".to_string(),
            item_type: components::ItemType::Potion,
        };
        let item_entity = app.world_mut().spawn(item.clone()).id();

        // 収集イベントを送信
        app.world_mut()
            .send_event(events::GameEvent::Collect(item_entity));

        app.update();

        // インベントリにアイテムが追加されたことを確認
        let inventory = app.world().resource::<resources::Inventory>();
        assert_eq!(inventory.items.len(), 1);
        assert_eq!(inventory.items[0].name, "テストアイテム");

        // エンティティが削除されたことを確認
        assert!(app.world().get_entity(item_entity).is_err());
    }
}
