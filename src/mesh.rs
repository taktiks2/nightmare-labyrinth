use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // NOTE: meshやプラグインをクリック検知するのに必要
    app.add_plugins(MeshPickingPlugin);
}
