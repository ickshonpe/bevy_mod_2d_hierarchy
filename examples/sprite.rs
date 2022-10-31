use bevy::math::vec2;
use bevy::prelude::Camera2dBundle;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_2d_transform_hierarchy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let map_size = vec2(16., 16.) * vec2(50 as f32, 30 as f32);
    let map_translation = - 0.5 * map_size;
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle2 {
        sprite: Sprite { 
            color: Color::RED, 
            //anchor: Anchor::TopRight,
            custom_size: Some(vec2(1000., 1000.)),
            ..Default::default() 
        },
        texture: asset_server.load("wall.png"),
        transform_2d: Transform2d::from_translation(map_translation).with_z(500.),
        ..Default::default()
    });
}

pub fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(Transform2dPlugin)
    .add_startup_system(setup)
    .run();
}