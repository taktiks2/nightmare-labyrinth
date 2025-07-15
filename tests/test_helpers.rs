use bevy::prelude::*;
use nightmare_labyrinth::AppPlugin;

/// テスト用のBevyアプリを作成するヘルパー関数
pub fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

/// ゲーム用のテストアプリを作成するヘルパー関数
pub fn create_game_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(AppPlugin);
    app
}

/// テスト用のワールドを作成するヘルパー関数
pub fn create_test_world() -> World {
    World::new()
}

/// 指定した回数だけアプリを更新するヘルパー関数
pub fn update_app_n_times(app: &mut App, n: usize) {
    for _ in 0..n {
        app.update();
    }
}

/// テスト用の単純なコンポーネント
#[derive(Component, Debug, PartialEq)]
pub struct TestComponent {
    pub value: i32,
}

impl TestComponent {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

/// テスト用のイベント
#[derive(Event, Debug, PartialEq, Clone)]
pub struct TestEvent {
    pub message: String,
}

impl TestEvent {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
