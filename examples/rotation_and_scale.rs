use bevy::prelude::*;
use bevy_mod_2d_hierarchy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Center;

#[derive(Component)]
pub struct Red;

#[derive(Component)]
pub struct White;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let s = 64f32;
    let d = 3. * s * Vec2::X;
    let n = 7;
    let center_id = commands
        .spawn(SpatialBundle2::default())
        .insert(Center)
        .id();
    for i in 0..n {
        let angle = i as f32 * (n as f32).recip() * PI;
        let translation = Mat2::from_angle(angle) * d * (1. - 2. * (i % 2) as f32);
        let sprite_id = commands
            .spawn(SpriteBundle2 {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(s * Vec2::ONE),
                    ..Default::default()
                },
                texture: asset_server.load("sprite.png"),
                transform2: Transform2::from_translation(-translation).with_depth(i as f32),
                ..Default::default()
            })
            .insert(White)
            .id();
        let red_sprite_id = commands
            .spawn(SpriteBundle2 {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(s * Vec2::ONE),
                    ..Default::default()
                },
                texture: asset_server.load("sprite.png"),
                transform2: Transform2::from_translation(translation).with_depth(i as f32),
                propagate: PropagateTransform2::TRANSLATION,
                ..Default::default()
            })
            .insert(Red)
            .id();
        commands
            .entity(center_id)
            .push_children(&[sprite_id, red_sprite_id]);
    }
}

fn update(
    time: Res<Time>,
    mut point_query: Query<&mut Transform2, (With<Center>, Without<Red>, Without<White>)>,
    mut red_query: Query<&mut Transform2, With<Red>>,
) {
    point_query.for_each_mut(|mut transform| {
        transform.rotate(0.3 * time.delta_seconds());
        transform.scale = 1. + time.elapsed_seconds().sin();
    });
    red_query.for_each_mut(|mut transform| {
        transform.rotation = 0.2 * (2.5 * time.elapsed_seconds()).sin();
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(Transform2dPlugin)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}
