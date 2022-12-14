use bevy::prelude::*;
use bevy_mod_2d_hierarchy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle2::default());
    let image_handle: Handle<Image> = asset_server.load("sprite.png");

    commands
        .spawn(SpriteBundle2 {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            texture: image_handle.clone(),
            transform2: Transform2::from_xy(-200., -200.),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(SpriteBundle2 {
                sprite: Sprite {
                    color: Color::RED,
                    ..Default::default()
                },
                texture: image_handle.clone(),
                transform2: Transform2::from_xy(16., 16.),
                ..Default::default()
            });

            builder.spawn(SpriteBundle2 {
                sprite: Sprite {
                    color: Color::BLUE,
                    ..Default::default()
                },
                texture: image_handle,
                transform2: Transform2::from_xy(-16., -16.),
                ..Default::default()
            });
        });
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Transform2dPlugin)
        .add_startup_system(setup)
        .run();
}
