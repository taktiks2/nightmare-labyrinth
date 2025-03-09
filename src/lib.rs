use bevy::prelude::*;

mod camera;
mod default;
mod dev_tools;
mod game;
mod globals;
mod mesh;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((default::plugin, camera::plugin, mesh::plugin, game::plugin));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}
