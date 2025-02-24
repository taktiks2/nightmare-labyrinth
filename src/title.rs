use bevy::{color::palettes::css::*, prelude::*};

use crate::states;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(states::GameState::Title),
            (setup_title_camera, setup_title),
        );
    }
}

fn setup_title(mut commands: Commands) {
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
                Text::new("Title"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
            ));
            p.spawn((
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(200.),
                    height: Val::Px(60.),
                    ..default()
                },
                BackgroundColor(BLACK.into()),
                BorderRadius::px(5., 5., 5., 5.),
                Button,
            ))
            .observe(handle_click)
            .with_children(|p| {
                p.spawn((
                    Text::new("Start"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                ));
            });
        });
}

fn setup_title_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Name::new("title_camera"),
        StateScoped(states::GameState::Title), // NOTE: stateが変わるとワールドから削除できる
    ));
}

pub fn handle_click(
    _click: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<states::GameState>>,
) {
    next_state.set(states::GameState::Playing);
}
