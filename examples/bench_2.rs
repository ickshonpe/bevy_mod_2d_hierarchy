use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_2d_transform_hierarchy::prelude::*;
use bevy_despawn_with::DespawnAllCommandsExt;

const N: u64 = 1_000_000;

fn spawn_2d_transform_hierarchy(
    mut commands: Commands,
) {
    println!("with Transform2d");
    commands.despawn_all::<With<GlobalTransform>>();
    commands.despawn_all::<With<Transform2d>>();
    commands.despawn_all::<With<Transform>>();
    for n in 0..N {
        commands.spawn_bundle((
            Transform2d::from_xy(n as f32, 0.0),
            GlobalTransform2d::default(),
            Visibility::default(),
            ComputedVisibility::default(),
        ));
    }
}

fn spawn_2d_transform_hierarchy_global(
    mut commands: Commands,
) {
    println!("with Transform2d + GlobalTransform");
    commands.despawn_all::<With<GlobalTransform>>();
    commands.despawn_all::<With<Transform2d>>();
    commands.despawn_all::<With<Transform>>();
    for n in 0..N {
        commands.spawn_bundle(Spatial2dBundle {
            transform_2d: Transform2d::from_xy(n as f32, 0.0),
            ..Default::default()
        });
    }
}

fn spawn_regular_transform_hierarchy(
    mut commands: Commands,
) {
    println!("with Transform");
    commands.despawn_all::<With<GlobalTransform>>();
    commands.despawn_all::<With<Transform2d>>();
    commands.despawn_all::<With<Transform>>();
    for n in 0..N {
        commands.spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(n as f32, 0.0, 0.0),
            ..Default::default()
        });
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
                AppState::None => AppState::Transform2d,
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