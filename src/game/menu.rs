use bevy::{color::palettes::css::*, prelude::*};

use crate::globals;

const SIDE_MENU_WIDTH: f32 = 224.;
const BOTTOM_TAB_HEIGHT: f32 = 48.;

pub fn setup_menu(mut commands: Commands) {
    // NOTE: Side Menu
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                right: Val::Px(0.),
                width: Val::Px(SIDE_MENU_WIDTH),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(GRAY.into()),
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(80.),
                    height: Val::Px(60.),
                    ..default()
                },
                BorderRadius::px(5., 5., 5., 5.),
                BackgroundColor(BLACK.into()),
            ));
        });

    // NOTE: Bottom Tab
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.),
                left: Val::Px(0.),
                width: Val::Px(globals::WINDOW_WIDTH - SIDE_MENU_WIDTH),
                height: Val::Px(BOTTOM_TAB_HEIGHT),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly, // 均等配置に変更
                ..default()
            },
            BackgroundColor(GRAY.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Attack: ".to_string()),
                TextFont {
                    // font: game_assets.font_regular.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(WHITE.into()),
            ));
            parent.spawn((
                Text::new("Defense: ".to_string()),
                TextFont {
                    // font: game_assets.font_regular.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(WHITE.into()),
            ));
            parent.spawn((
                Text::new("Gold: ".to_string()),
                TextFont {
                    // font: game_assets.font_regular.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(WHITE.into()),
            ));
        });
}
