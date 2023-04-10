use bevy::prelude::{Commands, Entity, EventWriter, Query, Transform, Without};

use crate::{
    components::{
        characters::{Character, ResourceInventory},
        jobs::Gatherer,
        resources::Gatherable,
        structures::{GridBody, StorageArea},
        tasks::{EmptyResourcesTask, GatherTask},
    },
    events::PointVisited,
};

type GatherCharacter = (
    &'static mut GatherTask,
    &'static mut Transform,
    &'static Character,
    Entity,
    &'static mut ResourceInventory,
);
type GatherableBody = (&'static Transform, &'static GridBody, &'static Gatherable);

pub fn do_gather_work(
    mut commands: Commands,
    mut query: Query<GatherCharacter>,
    resource_query: Query<GatherableBody, Without<Character>>,
    mut event_writer: EventWriter<PointVisited>,
) {
    for character_bundle in query.iter_mut() {
        let (mut gather_task, mut transform, _, entity, mut resource_inventory) = character_bundle;
        if gather_task.target.path_incomplete() {
            crate::utils::movement::visit_next_point(
                &mut gather_task.target.path,
                transform.as_mut(),
                &mut event_writer,
            );
        } else if let Some(entity) = gather_task.target.entity {
            if let Ok((_, _, gatherable)) = resource_query.get(entity) {
                resource_inventory.add_resource(&gatherable.resource);
                commands.entity(entity).despawn();
                gather_task.target.entity = None;
            }
        }

        if gather_task.is_complete() {
            commands
                .entity(entity)
                .remove::<GatherTask>()
                .remove::<Gatherer>();
        }
    }
}

type EmptyResourcesCharacter = (
    &'static mut EmptyResourcesTask,
    &'static mut Transform,
    &'static Character,
    Entity,
    &'static mut ResourceInventory,
);

pub fn do_empty_resources_work(
    mut commands: Commands,
    mut query: Query<EmptyResourcesCharacter>,
    mut storage_area_query: Query<&mut StorageArea>,
    mut event_writer: EventWriter<PointVisited>,
) {
    for character_bundle in query.iter_mut() {
        let (mut empty_task, mut transform, _, entity, mut resource_inventory) = character_bundle;

        if empty_task.target.path_incomplete() {
            crate::utils::movement::visit_next_point(
                &mut empty_task.target.path,
                transform.as_mut(),
                &mut event_writer,
            );
        } else if let Ok(mut storage_area) =
            storage_area_query.get_mut(empty_task.target.storage_area)
        {
            resource_inventory.empty_into(&mut storage_area);
            empty_task.target.done = true;
        } else {
            empty_task.target.done = true;
        }

        if empty_task.is_complete() {
            commands
                .entity(entity)
                .remove::<EmptyResourcesTask>()
                .remove::<Gatherer>();
        }
    }
}
