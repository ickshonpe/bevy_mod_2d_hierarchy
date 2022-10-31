use bevy::prelude::*;
use super::*;

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
        &Parent,
    )>,
    children_query: &Query<(&Children, Changed<Children>), (With<Parent>, With<GlobalTransform2d>)>,
    entity: Entity,
    expected_parent: Entity,
    mut changed: bool,
) -> Result<(), ()> {
    let global_matrix = {
        let (transform_2d, transform_changed, mut global_transform_2d, child_parent) =
            transform_query.get_mut(entity).map_err(drop)?;
        assert_eq!(
            child_parent.get(), expected_parent,
            "Malformed hierarchy. This probably means that your hierarchy has been improperly maintained, or contains a cycle"
        );
        changed |= transform_changed;
        if changed {
            *global_transform_2d = GlobalTransform2d(parent.mul_transform(*transform_2d));
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

pub fn derive_global_transform(
    mut query: Query<(&GlobalTransform2d, &mut GlobalTransform), (Changed<GlobalTransform2d>, Without<Transform>)>,
) {
    query.for_each_mut(|(
        global_transform_2d,
        mut global_transform
    )| {
        *global_transform = (*global_transform_2d).into();
    });
}

#[cfg(test)]
mod test {
    use bevy::ecs::system::CommandQueue;
    use bevy::math::vec2;
    use bevy::prelude::*;
    use bundles::Transform2dBundle;
    use super::*;

    #[derive(StageLabel)]
    struct Update;

    #[test]
    fn did_propagate() {
        let mut world = World::default();

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(transform_2d_propagate_system);

        let mut schedule = Schedule::default();
        schedule.add_stage(Update, update_stage);

        // Root entity
        world.spawn().insert_bundle(Transform2dBundle::from(Transform2d::from_xyz(1.0, 0.0, 0.0)));

        let mut children = Vec::new();
        world
            .spawn().insert_bundle(Transform2dBundle::from(Transform2d::from_xyz(1.0, 0.0, 0.0)))
            .with_children(|parent| {
                children.push(
                    parent
                        .spawn().insert_bundle(Transform2dBundle::from(Transform2d::from_xyz(0.0, 2.0, 0.)))
                        .id(),
                );
                children.push(
                    parent
                        .spawn().insert_bundle(Transform2dBundle::from(Transform2d::from_xyz(0.0, 0.0, 3.)))
                        .id(),
                );
            });
        schedule.run(&mut world);

        assert_eq!(
            *world.get::<GlobalTransform2d>(children[0]).unwrap(),
            GlobalTransform2d(Transform2d::from_xyz(1.0, 0.0, 0.0).mul_transform(Transform2d::from_xyz(0.0, 2.0, 0.0)))
        );

        assert_eq!(
            *world.get::<GlobalTransform2d>(children[1]).unwrap(),
            GlobalTransform2d(Transform2d::from_xyz(1.0, 0.0, 0.0).mul_transform(Transform2d::from_xyz(0.0, 0.0, 3.0)))
        );
    }

    #[test]
    fn did_propagate_command_buffer() {
        let mut world = World::default();
        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(transform_2d_propagate_system);

        let mut schedule = Schedule::default();
        schedule.add_stage(Update, update_stage);

        // Root entity
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &world);
        let mut children = Vec::new();
        commands
            .spawn().insert_bundle(Transform2dBundle::from(Transform2d::from_xyz(1.0, 0.0, 0.0)))
            .with_children(|parent| {
                children.push(
                    parent
                        .spawn().insert_bundle(Transform2dBundle::from(Transform2d::from_xyz(0.0, 2.0, 0.0)))
                        .id(),
                );
                children.push(
                    parent
                        .spawn().insert_bundle(Transform2dBundle::from(Transform2d::from_xyz(0.0, 0.0, 3.0)))
                        .id(),
                );
            });
        queue.apply(&mut world);
        schedule.run(&mut world);

        assert_eq!(
            *world.get::<GlobalTransform2d>(children[0]).unwrap(),
            GlobalTransform2d(GlobalTransform2d(Transform2d::from_xyz(1.0, 0.0, 0.0)).mul_transform(Transform2d::from_xyz(0.0, 2.0, 0.0)))
        );

        assert_eq!(
            *world.get::<GlobalTransform2d>(children[1]).unwrap(),
            GlobalTransform2d(Transform2d::from_xyz(1.0, 0.0, 0.0)).mul_transform(Transform2d::from_xyz(0.0, 0.0, 3.0)).into()
        );
    }

    #[test]
    fn correct_children() {
        let mut world = World::default();

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(transform_2d_propagate_system);

        let mut schedule = Schedule::default();
        schedule.add_stage(Update, update_stage);

        // Add parent entities
        let mut children = Vec::new();
        let parent = {
            let mut command_queue = CommandQueue::default();
            let mut commands = Commands::new(&mut command_queue, &world);
            let parent = commands.spawn().insert(Transform2d::from_xyz(1.0, 0.0, 0.0)).id();
            commands.entity(parent).with_children(|parent| {
                children.push(parent.spawn().insert(Transform2d::from_xyz(0.0, 2.0, 0.0)).id());
                children.push(parent.spawn().insert(Transform2d::from_xyz(0.0, 3.0, 0.0)).id());
            });
            command_queue.apply(&mut world);
            schedule.run(&mut world);
            parent
        };

        assert_eq!(
            world
                .get::<Children>(parent)
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            children,
        );

        // Parent `e1` to `e2`.
        {
            let mut command_queue = CommandQueue::default();
            let mut commands = Commands::new(&mut command_queue, &world);
            commands.entity(children[1]).add_child(children[0]);
            command_queue.apply(&mut world);
            schedule.run(&mut world);
        }

        assert_eq!(
            world
                .get::<Children>(parent)
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            vec![children[1]]
        );

        assert_eq!(
            world
                .get::<Children>(children[1])
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            vec![children[0]]
        );

        assert!(world.despawn(children[0]));

        schedule.run(&mut world);

        assert_eq!(
            world
                .get::<Children>(parent)
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            vec![children[1]]
        );
    }

    #[test]
    fn correct_transforms_when_no_children() {
        let mut app = App::new();

        app.add_system(transform_2d_propagate_system);

        let translation = vec2(1.0, 0.0);

        // These will be overwritten.
        let mut child = Entity::from_raw(0);
        let mut grandchild = Entity::from_raw(1);
        let parent = app
            .world
            .spawn()
            .insert_bundle((
                Transform2d::from_translation(translation),
                GlobalTransform2d::IDENTITY,
            ))
            .with_children(|builder| {
                child = builder
                    .spawn().insert_bundle(Transform2dBundle::IDENTITY)
                    .with_children(|builder| {
                        grandchild = builder.spawn().insert_bundle(Transform2dBundle::IDENTITY).id();
                    })
                    .id();
            })
            .id();

        app.update();

        // check the `Children` structure is spawned
        assert_eq!(&**app.world.get::<Children>(parent).unwrap(), &[child]);
        assert_eq!(&**app.world.get::<Children>(child).unwrap(), &[grandchild]);
        // Note that at this point, the `GlobalTransform`s will not have updated yet, due to `Commands` delay
        app.update();

        let mut state = app.world.query::<&GlobalTransform2d>();
        for global in state.iter(&app.world) {
            assert_eq!(global, &GlobalTransform2d(Transform2d::from_translation(translation)));
        }
    }

    #[test]
    #[should_panic]
    fn panic_when_hierarchy_cycle() {
        // We cannot directly edit Parent and Children, so we use a temp world to break
        // the hierarchy's invariants.
        let mut temp = World::new();
        let mut app = App::new();

        // Adding the system in a single threaded stage. As the system will panic, this will
        // only bring down the current test thread.
        app.add_stage("single", SystemStage::single_threaded())
            .add_system_to_stage("single", transform_2d_propagate_system);

        fn setup_world(world: &mut World) -> (Entity, Entity) {
            let mut grandchild = Entity::from_raw(0);
            let child = world
                .spawn().insert_bundle(Transform2dBundle::IDENTITY)
                .with_children(|builder| {
                    grandchild = builder.spawn().insert_bundle(Transform2dBundle::IDENTITY).id();
                })
                .id();
            (child, grandchild)
        }

        let (temp_child, temp_grandchild) = setup_world(&mut temp);
        let (child, grandchild) = setup_world(&mut app.world);

        assert_eq!(temp_child, child);
        assert_eq!(temp_grandchild, grandchild);

        app.world
            .spawn().insert_bundle(Transform2dBundle::IDENTITY)
            .push_children(&[child]);
        std::mem::swap(
            &mut *app.world.get_mut::<Parent>(child).unwrap(),
            &mut *temp.get_mut::<Parent>(grandchild).unwrap(),
        );

        app.update();
    }
}