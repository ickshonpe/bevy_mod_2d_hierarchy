use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("sprite.png");
    commands.spawn(Camera2dBundle::default());
    for x in 0..100 {
        for y in 0..100 {
            commands
                .spawn(SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_xyz(x as f32 * 16., y as f32 * 16., 0.),
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::YELLOW,
                            ..Default::default()
                        },
                        texture: texture.clone(),
                        transform: Transform::from_xyz(0., 32., 0.),
                        ..Default::default()
                    });
                });
        }
    }
}

pub fn update(time: Res<Time>, mut query: Query<&mut Transform>) {
    query.for_each_mut(|mut transform| {
        transform.rotate(Quat::from_rotation_z(time.delta_seconds()))
    });
}

pub fn update_2(time: Res<Time>, mut query: Query<&mut Transform, With<Parent>>) {
    query.for_each_mut(|mut transform| {
        transform.rotate(Quat::from_rotation_z(time.delta_seconds()))
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin { 
                window: WindowDescriptor {
                    present_mode: bevy::window::PresentMode::Immediate,
                    ..Default::default()
                }, 
                ..Default::default()
            }
        ))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn)
        .add_system(update)
        .add_system(update_2)
        .run();
}
