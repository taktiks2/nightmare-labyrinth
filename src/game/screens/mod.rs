use bevy::prelude::*;

mod home;
mod loading;
mod title;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((loading::plugin, title::plugin, home::plugin));
}
