use bevy::{color::palettes::css::*, prelude::*};

use crate::game::states;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(states::GameState::Loading), setup_loading);
}

fn setup_loading(mut commands: Commands) {
    commands
        .spawn((
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            Name::new("loading_screen"),
            BackgroundColor(GRAY.into()),
            StateScoped(states::GameState::Loading), // NOTE: stateが変わるとワールドから削除できる
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
            ));
        });
}
