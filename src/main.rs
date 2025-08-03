//! Nightmare Labyrinth - ホラー風ローグライクゲーム
//!
//! 悪夢から抜け出せない少女を救出するゲーム
//! - ランダム生成マップ
//! - ターン制システム
//! - 主人公は非戦闘
//! - 恐怖パラメータ（満腹度の代替）

use bevy::prelude::*;
use clap::Parser;

use nightmare_labyrinth::AppPlugin;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// デバッグ用初期状態指定
    #[arg(long, value_parser = parse_debug_state)]
    debug_state: Option<String>,
}

fn parse_debug_state(s: &str) -> Result<String, String> {
    match s {
        "loading" | "title" | "playing" | "menu" => Ok(s.to_string()),
        _ => Err(format!(
            "無効な状態: {}. 使用可能: loading, title, playing, menu",
            s
        )),
    }
}

/// メインエントリーポイント
///
/// Bevyアプリケーションを初期化し、メインプラグインを追加してゲームを開始
fn main() {
    let args = Args::parse();

    let mut app = App::new();
    app.add_plugins(AppPlugin);

    // デバッグ状態の設定
    if let Some(state_str) = args.debug_state {
        use nightmare_labyrinth::game::{resources::DebugConfig, states::GameState};

        let debug_state = match state_str.as_str() {
            "loading" => GameState::Loading,
            "title" => GameState::Title,
            "playing" => GameState::Playing,
            "menu" => GameState::Menu,
            _ => GameState::Loading, // フォールバック
        };

        app.insert_resource(DebugConfig {
            initial_state: Some(debug_state),
        });
    }

    app.run();
}
