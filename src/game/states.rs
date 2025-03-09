use bevy::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Playing,
    Menu,
}

pub(super) fn plugin(app: &mut App) {
    app.enable_state_scoped_entities::<GameState>() // NOTE: StateScopedを使うために必要、エンティティを自動で削除する
        .init_state::<GameState>();
}
