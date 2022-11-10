use std::f32::consts::PI;

use bevy::math::vec2;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_mod_2d_hierarchy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle2::default());
    let image_handle: Handle<Image> = asset_server.load("arrow.png");
    let sprite = Sprite {
        custom_size: Some(vec2(30.0, 40.0)),
        anchor: bevy::sprite::Anchor::BottomCenter,
        ..Default::default()
    };
    commands
        .spawn_bundle((
            sprite.clone(),
            Transform2::from_xy(-100., 0.),
            image_handle.clone(),
            GlobalTransform2::default(),
            GlobalTransform::default(),
            Visibility::default(),
            ComputedVisibility::default(),
            PropagateTransform2::ALL,
        ))
        .with_children(|builder| {
            builder.spawn_bundle((
                sprite.clone(),
                Transform2 {
                    rotation: PI / 4.,
                    scale: 2.0,
                    ..Default::default()
                },
                image_handle.clone(),
                GlobalTransform2::default(),
                GlobalTransform::default(),
                Visibility::default(),
                ComputedVisibility::default(),
                PropagateTransform2::ALL,
            ));
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: sprite.clone(),
            transform: Transform::from_xyz(100., 0., 0.),
            texture: image_handle.clone(),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn_bundle(SpriteBundle {
                sprite: sprite.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(PI / 4.),
                    scale: vec3(2.0, 2.0, 1.0),
                    ..Default::default()
                },
                texture: image_handle.clone(),
                ..Default::default()
            });
        });
}

pub fn rotation(
    time: Res<Time>,
    mut tf: Query<&mut Transform, (With<Sprite>, With<Children>)>,
    mut tf2d: Query<&mut Transform2, (With<Sprite>, With<Children>)>,
) {
    let angle = 0.5 * time.delta_seconds();
    tf.for_each_mut(|mut tf| {
        tf.rotate_z(angle);
    });
    tf2d.for_each_mut(|mut tf| {
        tf.rotate(angle);
    });
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Transform2dPlugin)
        .add_startup_system(setup)
        .add_system(rotation)
        .run();
}
