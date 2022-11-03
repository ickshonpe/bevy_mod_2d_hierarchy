pub mod bundles;
pub mod systems;
pub mod transform2;

use bevy::prelude::*;
use transform2::GlobalTransform2;
use transform2::Propagate;
use transform2::Transform2;

pub mod prelude {
    pub use crate::bundles::*;
    pub use crate::transform2::GlobalTransform2;
    pub use crate::transform2::Propagate;
    pub use crate::transform2::Transform2;
    pub use crate::Hierarchy2dPlugin;
}

/// Label enum for the systems relating to transform propagation
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum Hierarchy2dSystem {
    /// Propagates changes in transform to children's [`GlobalTransform`](crate::components::GlobalTransform)
    Transform2Propagate,
    DeriveGlobalTransform,
}

/// The base plugin for handling [`Transform`] components
#[derive(Default)]
pub struct Hierarchy2dPlugin;

impl Plugin for Hierarchy2dPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Transform2>()
            .register_type::<GlobalTransform2>()
            .register_type::<Propagate>()
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                systems::transform_2d_propagate_system
                    .label(Hierarchy2dSystem::Transform2Propagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                systems::transform_2d_propagate_system
                    .label(Hierarchy2dSystem::Transform2Propagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                systems::derive_global_transform.label(Hierarchy2dSystem::DeriveGlobalTransform),
            );
    }
}
