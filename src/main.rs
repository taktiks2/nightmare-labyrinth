use bevy::{asset::AssetMetaCheck, log::LogPlugin, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::ops::Neg;

mod components;
mod events;
mod game;
mod globals;
mod input;
mod loading;
mod resources;
mod states;
mod title;

use globals::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Nightmare Labyrinth".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    // NOTE: wasmビルドでassetファイルを読み込むのに必要
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()) // NOTE: 画像がぼやけないようにする
                .set(LogPlugin {
                    // NOTE: bevy標準のログを出力するのに必要
                    filter: "nightmare_labyrinth".to_string(), // NOTE: 自身の出したログのみ出力するさせる
                    ..default()
                }),
        )
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(MeshPickingPlugin) // NOTE: meshやプラグインをクリック検知するのに必要
        .add_plugins(WorldInspectorPlugin::new()) // NOTE: インスペクタープラグイン
        .enable_state_scoped_entities::<states::GameState>() // NOTE: StateScopedを使うために必要、エンティティを自動で削除する
        .init_state::<states::GameState>()
        .add_plugins(game::GamePlugin)
        .add_plugins(loading::LoadingPlugin)
        .add_plugins(title::TitlePlugin)
        .add_loading_state(
            // NOTE: アセットがロードされるまでローディングを出す
            LoadingState::new(states::GameState::Loading)
                .continue_to_state(states::GameState::Title)
                .load_collection::<resources::GameAssets>(),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(Update, input::handle_keyboard_input)
        .add_event::<events::GameEvent>()
        .add_event::<events::InputEvent>()
        .add_event::<events::GameTick>()
        .run();
}

fn calculate_offset(size: f32) -> f32 {
    0.5 * SPRITE_SCALE * SPRITE_SIZE * (size - 1.)
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Name::new("main camera"),
        Transform::from_translation(Vec3::new(
            calculate_offset(BOARD_WIDTH as f32),
            calculate_offset(BOARD_HEIGHT as f32).neg(),
            0.,
        )), // NOTE: 左上から座標が始まるようにnegにする
    ));
}
