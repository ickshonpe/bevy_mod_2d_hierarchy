

pub mod transform_2d;
pub mod systems;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use transform_2d::GlobalTransform2d;
use transform_2d::Transform2d;

#[derive(Bundle, Clone, Copy, Debug, Default)]
pub struct Transform2dBundle {
    pub local_2d: Transform2d,
    pub global_2d: GlobalTransform2d,
    pub global: GlobalTransform
}

impl Transform2dBundle {
    pub const IDENTITY: Self = Transform2dBundle {
        local_2d: Transform2d::IDENTITY,
        global_2d: GlobalTransform2d::IDENTITY,
        global: GlobalTransform::identity()
    };

    #[inline]
    pub const fn from_transform(transform: Transform2d) -> Self {
        Transform2dBundle {
            local_2d: transform,
            ..Self::IDENTITY
        }
    }
}

impl From<Transform2d> for Transform2dBundle {
    #[inline]
    fn from(transform: Transform2d) -> Self {
        Self::from_transform(transform)
    }
}
/// Label enum for the systems relating to transform propagation
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum Transform2dSystem {
    /// Propagates changes in transform to children's [`GlobalTransform`](crate::components::GlobalTransform)
    Transform2dPropagate,
    DeriveGlobalTransform,
}

/// The base plugin for handling [`Transform`] components
#[derive(Default)]
pub struct Transform2dPlugin;

impl Plugin for Transform2dPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Transform2d>()
            .register_type::<GlobalTransform2d>()
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                systems::transform_2d_propagate_system.label(Transform2dSystem::Transform2dPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                systems::transform_2d_propagate_system.label(Transform2dSystem::Transform2dPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                systems::derive_global_transform.label(Transform2dSystem::DeriveGlobalTransform),
            );

    }
}