use bevy::prelude::*;
use super::*;

#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Transform2dPropagationDescriptor {
    pub inherit_translation: bool,
    pub inherit_z: bool,
    pub inherit_rotation: bool,
    pub inherit_scale: bool,
}

/// Update [`GlobalTransform`] component of entities based on entity hierarchy and
/// [`Transform`] component.
pub fn transform_2d_propagate_system(
    mut root_query: Query<
        (
            Option<(&Children, Changed<Children>)>,
            &Transform2d,
            Changed<Transform2d>,
            &mut GlobalTransform2d,
            Entity,
        ),
        (
            Without<Parent>,
        )
    >,
    mut transform_query: Query<(
        &Transform2d,
        Changed<Transform2d>,
        &mut GlobalTransform2d,
        Option<&Transform2dPropagationDescriptor>,
        &Parent,
    )>,
    children_query: Query<(&Children, Changed<Children>), (With<Parent>, With<GlobalTransform2d>)>,
) {
    for (children, transform_2d, transform_2d_changed, mut global_transform_2d, entity) in
        root_query.iter_mut()
    {
        let mut changed = transform_2d_changed;
        if transform_2d_changed {
            *global_transform_2d = GlobalTransform2d(*transform_2d);
        }

        if let Some((children, changed_children)) = children {
            changed |= changed_children;
            for child in children {
                let _ = propagate_recursive(
                    &global_transform_2d,
                    &mut transform_query,
                    &children_query,
                    *child,
                    entity,
                    changed,
                );
            }
        }
    }
}

fn propagate_recursive(
    parent: &GlobalTransform2d,
    transform_query: &mut Query<(
        &Transform2d,
        Changed<Transform2d>,
        &mut GlobalTransform2d,
        Option<&Transform2dPropagationDescriptor>,
        &Parent,
    )>,
    children_query: &Query<(&Children, Changed<Children>), (With<Parent>, With<GlobalTransform2d>)>,
    entity: Entity,
    expected_parent: Entity,
    mut changed: bool,
) -> Result<(), ()> {
    let global_matrix = {
        let (transform_2d, transform_changed, mut global_transform_2d, desc, child_parent) =
            transform_query.get_mut(entity).map_err(drop)?;
        assert_eq!(
            child_parent.get(), expected_parent,
            "Malformed hierarchy. This probably means that your hierarchy has been improperly maintained, or contains a cycle"
        );
        changed |= transform_changed;
        if changed {
            *global_transform_2d = parent.propagate_transform(*transform_2d, desc);
        }
        *global_transform_2d
    };
    let (children, changed_children) = children_query.get(entity).map_err(drop)?;
    changed |= changed_children;
    for child in children {
        let _ = propagate_recursive(
            &global_matrix,
            transform_query,
            children_query,
            *child,
            entity,
            changed,
        );
    }
    Ok(())
}

