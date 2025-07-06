//! ローディング画面システム
//!
//! ゲーム起動時のアセットロード中に表示されるローディング画面
//! シンプルなローディングテキストを表示し、ユーザーにロード状態を通知
//! Loading状態時のみ有効で、状態変化時に自動的にクリーンアップ

use bevy::{color::palettes::css::*, prelude::*};

use crate::game::states;

/// ローディング画面プラグイン
///
/// Loading状態に入った時にローディングUIを表示するシステムを登録
/// アセットのロードや初期化中にユーザーにフィードバックを提供
pub(super) fn plugin(app: &mut App) {
    // Loading状態に入った時にローディング画面をセットアップ
    app.add_systems(OnEnter(states::GameState::Loading), setup_loading);
}

/// ローディング画面のUIをセットアップ
///
/// 画面中央に「Loading」テキストを表示するシンプルなUIを作成
/// 背景色はグレーで、テキストは大きなフォントで表示
/// StateScopedで状態変化時に自動削除される
///
/// # 引数
/// * `commands` - EntityとComponentをスポーンするためのコマンド
fn setup_loading(mut commands: Commands) {
    // メインコンテナを作成（フルスクリーンで中央配置）
    commands
        .spawn((
            Node {
                // コンテンツを中央に配置
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                // フルスクリーンサイズ
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            Name::new("loading_screen"),
            // グレーの背景色
            BackgroundColor(GRAY.into()),
            // Loading状態のみで存在し、状態変化時に自動削除
            StateScoped(states::GameState::Loading),
        ))
        .with_children(|parent| {
            // ローディングテキストを追加
            parent.spawn((
                Text::new("Loading"),
                TextFont {
                    // 大きなフォントサイズで表示
                    font_size: 60.0,
                    ..default()
                },
            ));
        });
}
