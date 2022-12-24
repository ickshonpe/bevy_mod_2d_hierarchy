pub mod bundles;
pub mod systems;
pub mod transform2;

use bevy::prelude::*;
use transform2::GlobalTransform2;
use transform2::PropagateTransform2;
use transform2::Transform2;

pub mod prelude {
    pub use crate::bundles::*;
    pub use crate::transform2::GlobalTransform2;
    pub use crate::transform2::PropagateTransform2;
    pub use crate::transform2::Transform2;
    pub use crate::Transform2dPlugin;
}

/// Label enum for the systems relating to transform propagation
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum Transform2dSystem {
    /// Propagates changes in transform to children's [`GlobalTransform`](crate::components::GlobalTransform)
    PropagateTransform2System,
    DeriveGlobalTransformSystem,
}

/// The base plugin for handling [`Transform`] components
#[derive(Default)]
pub struct Transform2dPlugin;

impl Plugin for Transform2dPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Transform2>()
            .register_type::<GlobalTransform2>()
            .register_type::<PropagateTransform2>()
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                systems::transform_2d_propagate_system
                    .label(Transform2dSystem::PropagateTransform2System),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                systems::transform_2d_propagate_system
                    .label(Transform2dSystem::PropagateTransform2System),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                systems::derive_global_transform
                    .label(Transform2dSystem::DeriveGlobalTransformSystem),
            );
    }
}
