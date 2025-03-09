use bevy::{color::palettes::css::*, prelude::*};
use std::path::Path;

use crate::game::{events, resources, states};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(states::GameState::Title), setup_title);
}

fn setup_title(mut commands: Commands, game_assets: Res<resources::GameAssets>) {
    let save_file_exists = Path::new("assets/save.json").exists();

    commands
        .spawn((
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect {
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    top: Val::Px(100.),
                    bottom: Val::Px(100.),
                },
                ..default()
            },
            Name::new("title_screen"),
            StateScoped(states::GameState::Title), // NOTE: stateが変わるとワールドから削除できる
            BackgroundColor(GRAY.into()),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("悪夢の迷宮"),
                TextFont {
                    font: game_assets.font_bold.clone(),
                    font_size: 60.0,
                    ..default()
                },
            ));
            if save_file_exists {
                p.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(240.),
                        height: Val::Px(60.),
                        ..default()
                    },
                    BackgroundColor(BLACK.into()),
                    BorderRadius::px(5., 5., 5., 5.),
                    Button,
                ))
                .observe(handle_continue_click)
                .with_children(|p| {
                    p.spawn((
                        Text::new("Continue"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                    ));
                });
            }
            p.spawn((
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(240.),
                    height: Val::Px(60.),
                    ..default()
                },
                BackgroundColor(BLACK.into()),
                BorderRadius::px(5., 5., 5., 5.),
                Button,
            ))
            .observe(handle_new_game_click)
            .with_children(|p| {
                p.spawn((
                    Text::new("New Game"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                ));
            });
        });
}

pub fn handle_continue_click(
    _click: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<states::GameState>>,
    mut load_save_events: EventWriter<events::LoadSaveEvent>,
) {
    load_save_events.send(events::LoadSaveEvent);
    next_state.set(states::GameState::Playing);
}

pub fn handle_new_game_click(
    _click: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<states::GameState>>,
) {
    next_state.set(states::GameState::Playing);
}
