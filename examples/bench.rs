use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_2d_transform_hierarchy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_despawn_with::DespawnAllCommandsExt;

const N: u64 = 1000;

fn spawn_2d_transform_hierarchy(
    mut commands: Commands,
) {
    println!("with Transform2d");
    commands.despawn_all::<With<GlobalTransform>>();
    for n in 0..N {
        let mut parent = commands.spawn_bundle((
                Transform2d::from_xy(n as f32, 0.0),
                GlobalTransform2d::default(),
                Visibility::default(),
                ComputedVisibility::default(),
            ))
            .id();

        for n in 0..N {
            let child = commands.spawn_bundle((
                    Transform2d {
                        translation: vec2(0.0, n as f32),
                        z: 0.1,
                        rotation: 1.0,
                        scale: 1.5,
                    },
                    GlobalTransform2d::default(),
                    Visibility::default(),
                    ComputedVisibility::default(),
                ))
                .id();
            commands.entity(parent).add_child(child);
            parent = child;
        }
    }
}

fn spawn_2d_transform_hierarchy_global(
    mut commands: Commands,
) {
    println!("with Transform2d + GlobalTransform");
    commands.despawn_all::<With<Transform2d>>();
    commands.despawn_all::<With<Transform>>();
    for n in 0..N {
        let mut parent = commands.spawn_bundle(Spatial2dBundle {
                transform_2d: Transform2d::from_xy(n as f32, 0.0),
                ..Default::default()
            })
            .id();

        for n in 0..N {
            let child = commands.spawn_bundle(Spatial2dBundle {
                    transform_2d: Transform2d {
                        translation: vec2(0.0, n as f32),
                        z: 0.1,
                        rotation: 1.0,
                        scale: 1.5,
                    },
                    ..Default::default()
                })
                .id();
            commands.entity(parent).add_child(child);
            parent = child;
        }
    }
}

fn spawn_regular_transform_hierarchy(
    mut commands: Commands,
) {
    println!("with Transform");
    commands.despawn_all::<With<Transform2d>>();
    for n in 0..N {
        let mut parent = commands.spawn_bundle(SpatialBundle {
                transform: Transform::from_xyz(n as f32, 0.0, 0.0),
                ..Default::default()
            })
            .id();

        for n in 0..N {
            let child = commands.spawn_bundle(SpatialBundle {
                    transform: Transform {
                        translation: vec3(0.0, n as f32, 0.1),
                        rotation: Quat::from_rotation_z(1.0),
                        scale: vec3(1.5, 1.5, 1.0),
                    },
                    ..Default::default()
                })
                .id();
            commands.entity(parent).add_child(child);
            parent = child;
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum AppState {
    None,
    Transform,
    Transform2d,
    Transform2dGlobal,
}

fn switcher(
    mut state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let next = 
            match state.current() {
                AppState::None => AppState::Transform,
                AppState::Transform => AppState::Transform2d,
                AppState::Transform2d => AppState::Transform2dGlobal,
                AppState::Transform2dGlobal => AppState::Transform,
            };
        let _ = state.set(next);
    }
}

pub fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        present_mode: bevy::window::PresentMode::Immediate,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(Transform2dPlugin)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_state(AppState::None)
    .add_system_set(
        SystemSet::on_enter(AppState::Transform)
        .with_system(spawn_regular_transform_hierarchy)
    )
    .add_system_set(
        SystemSet::on_enter(AppState::Transform2d)
        .with_system(spawn_2d_transform_hierarchy)
    )
    .add_system_set(
        SystemSet::on_enter(AppState::Transform2dGlobal)
        .with_system(spawn_2d_transform_hierarchy_global)
    )
    .add_system(switcher)
    .run();
}