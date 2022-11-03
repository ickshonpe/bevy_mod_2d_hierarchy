pub mod transform2;
pub mod systems;
pub mod bundles;

use bevy::prelude::*;
use transform2::GlobalTransform2;
use transform2::Propagate;
use transform2::Transform2;

pub mod prelude {
    pub use crate::transform2::Propagate;
    pub use crate::transform2::Transform2;
    pub use crate::transform2::GlobalTransform2;
    pub use crate::Transform2Plugin;
    pub use crate::bundles::*;
}

/// Label enum for the systems relating to transform propagation
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum Transform2System {
    /// Propagates changes in transform to children's [`GlobalTransform`](crate::components::GlobalTransform)
    Transform2dPropagate,
    DeriveGlobalTransform,
}

/// The base plugin for handling [`Transform`] components
#[derive(Default)]
pub struct Transform2Plugin;

impl Plugin for Transform2Plugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Transform2>()
            .register_type::<GlobalTransform2>()
            .register_type::<Propagate>()
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                systems::transform_2d_propagate_system.label(Transform2System::Transform2dPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                systems::transform_2d_propagate_system.label(Transform2System::Transform2dPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                systems::derive_global_transform.label(Transform2System::DeriveGlobalTransform),
            );

    }
}