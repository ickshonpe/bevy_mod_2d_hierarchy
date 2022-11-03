use bevy::prelude::*;
use super::*;

/// Update [`GlobalTransform2`] component of entities based on entity hierarchy and
/// [`Transform2`] component.
pub fn transform_2d_propagate_system(
    mut root_query: Query<
        (
            Option<(&Children, Changed<Children>)>,
            &Transform2,
            Changed<Transform2>,
            &mut GlobalTransform2,
            Entity,
        ),
        (
            Without<Parent>,
        )
    >,
    mut transform_query: Query<(
        &Transform2,
        Changed<Transform2>,
        &mut GlobalTransform2,
        &Propagate,
        &Parent,
    )>,
    children_query: Query<(&Children, Changed<Children>), (With<Parent>, With<GlobalTransform2>)>,
) {
    for (children, transform_2d, transform_2d_changed, mut global_transform_2d, entity) in
        root_query.iter_mut()
    {
        let mut changed = transform_2d_changed;
        if transform_2d_changed {
            *global_transform_2d = (*transform_2d).into();
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
    parent: &GlobalTransform2,
    transform_query: &mut Query<(
        &Transform2,
        Changed<Transform2>,
        &mut GlobalTransform2,
        &Propagate,
        &Parent,
    )>,
    children_query: &Query<(&Children, Changed<Children>), (With<Parent>, With<GlobalTransform2>)>,
    entity: Entity,
    expected_parent: Entity,
    mut changed: bool,
) -> Result<(), ()> {
    let global_matrix = {
        let (transform2, transform_changed, mut global_transform2, propagate, child_parent) =
            transform_query.get_mut(entity).map_err(drop)?;
        assert_eq!(
            child_parent.get(), expected_parent,
            "Malformed hierarchy. This probably means that your hierarchy has been improperly maintained, or contains a cycle"
        );
        changed |= transform_changed;
        if changed {
            *global_transform2 = parent.propagate_transform(*transform2, *propagate);
        }
        *global_transform2
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

pub fn derive_global_transform(
    mut query: Query<(&GlobalTransform2, &mut GlobalTransform), (Changed<GlobalTransform2>, Without<Transform>)>,
) {
    query.for_each_mut(|(
        global_transform_2d,
        mut global_transform
    )| {
        *global_transform = (*global_transform_2d).into();
    });
}