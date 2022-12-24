use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_mod_2d_hierarchy::prelude::*;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("sprite.png");
    commands.spawn(Camera2dBundle2::default());
    for x in 0..100 {
        for y in 0..100 {
            commands
                .spawn(SpriteBundle2 {
                    texture: texture.clone(),
                    transform2: Transform2::from_xy(x as f32 * 16., y as f32 * 16.),
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn(SpriteBundle2 {
                        sprite: Sprite {
                            color: Color::YELLOW,
                            ..Default::default()
                        },
                        texture: texture.clone(),
                        transform2: Transform2::from_xy(0., 32.),
                        ..Default::default()
                    });
                });
        }
    }
}

pub fn update(time: Res<Time>, mut query: Query<&mut Transform2>) {
    query.for_each_mut(|mut transform| transform.rotate(time.delta_seconds()));
}

pub fn update_2(time: Res<Time>, mut query: Query<&mut Transform2, With<Parent>>) {
    query.for_each_mut(|mut transform| transform.rotate(time.delta_seconds()));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                present_mode: bevy::window::PresentMode::Immediate,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(Transform2dPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn)
        .add_system(update)
        .add_system(update_2)
        .run();
}
