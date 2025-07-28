use bevy::prelude::*;
use nightmare_labyrinth::AppPlugin;

#[test]
fn test_app_creation() {
    // ヘッドレスモードでアプリを作成
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // 基本リソースが存在することを確認
    assert!(app.world().contains_resource::<Time>());
}

#[test]
fn test_basic_systems() {
    // ヘッドレスモードでアプリを作成し、基本的な実行を確認
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // 基本的なシステムの実行を確認
    app.update();

    // アプリケーションが1フレーム実行されることを確認
    assert!(app.world().contains_resource::<Time>());
}
