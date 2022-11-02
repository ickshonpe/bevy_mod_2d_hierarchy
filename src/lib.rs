pub mod transform_2d;
pub mod systems;
mod bundles;
pub mod alt_systems;

use bevy::prelude::*;
use transform_2d::GlobalTransform2d;
use transform_2d::Transform2d;

pub mod prelude {
    pub use crate::transform_2d::Transform2d;
    pub use crate::transform_2d::GlobalTransform2d;
    pub use crate::Transform2dPlugin;
    pub use crate::bundles::*;
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


/// The base plugin for handling [`Transform`] components
#[derive(Default)]
pub struct AltTransform2dPlugin;

impl Plugin for AltTransform2dPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Transform2d>()
            .register_type::<GlobalTransform2d>()
            .register_type::<alt_systems::Transform2dPropagationDescriptor>()
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                alt_systems::transform_2d_propagate_system.label(Transform2dSystem::Transform2dPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                alt_systems::transform_2d_propagate_system.label(Transform2dSystem::Transform2dPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                systems::derive_global_transform.label(Transform2dSystem::DeriveGlobalTransform),
            );

    }
}