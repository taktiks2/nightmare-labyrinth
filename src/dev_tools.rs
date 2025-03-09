use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(super) fn plugin(app: &mut App) {
    // NOTE: インスペクタープラグイン
    app.add_plugins(WorldInspectorPlugin::new());
}
