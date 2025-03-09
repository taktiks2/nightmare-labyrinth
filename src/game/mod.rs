use bevy::prelude::*;

mod actions;
mod assets;
mod components;
mod events;
mod input;
mod resources;
mod save;
mod screens;
mod states;
mod turn_base;
mod utils;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        events::plugin,
        states::plugin,
        resources::plugin,
        screens::plugin,
    ));
}
