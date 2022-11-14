use bevy::prelude::*;
use bevy_mod_2d_hierarchy::prelude::*;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("sprite.png");
    commands.spawn(Camera2dBundle2::default());
    commands
        .spawn(SpriteBundle2 {
            texture: texture.clone(),
            transform2: Transform2::from_rotation(0.5 * std::f32::consts::PI).with_scale(3.),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(SpriteBundle2 {
                sprite: Sprite {
                    color: Color::YELLOW,
                    ..Default::default()
                },
                texture,
                transform2: Transform2::from_xy(0., 32.),
                propagate: PropagateTransform2::TRANSLATION,
                ..Default::default()
            });
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Transform2dPlugin)
        .add_startup_system(spawn)
        .run();
}
