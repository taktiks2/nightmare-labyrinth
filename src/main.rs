use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_sprite))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("tak-cat.png"),
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        Transform::from_xyz(10., 0., 0.),
    ));
}
