use bevy::prelude::{Commands, Entity, Query, Transform, Visibility};

use crate::{
    components::{
        characters::{Character, ResourceInventory},
        jobs::{Gatherer, Job, ManualAssignment},
        movement::{Path, VisitedPoint},
        resources::Gatherable,
        structures::{GridBody, StorageArea},
        tasks::{
            EmptyResourcesTarget, EmptyResourcesTask, GatherTask, GatheringTarget, WithoutTask,
        },
        GridBox, Map,
    },
    resources::config::grid::{grid_coordinate_from_world, pathfind},
};

type GathererTransform = (
    &'static Character,
    Entity,
    &'static Transform,
    &'static ResourceInventory,
    &'static Gatherer,
    &'static ManualAssignment,
);

pub fn assign_gatherer_task(
    mut commands: Commands,
    query: Query<GathererTransform, WithoutTask>,
    mut gatherable_query: Query<(&mut Gatherable, &GridBody, &Visibility, Entity)>,
    storage_area_query: Query<(&StorageArea, &GridBody, Entity)>,
    map_query: Query<&Map>,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };

    let mut used_gatherables: Vec<Entity> = Vec::new();

    for character_bundle in query.iter() {
        let (character, entity, transform, resource_inventory, gatherer, manual_assignment) =
            character_bundle;

        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = character.visibility_box(character_coordinate);

        let storage_area_bundle = storage_area_query
            .iter()
            .min_by_key(|(_, body, _)| body.center_coordinate.distance(&character_coordinate));

        let possible_task = build_gatherer_task(
            &visibility_box,
            resource_inventory,
            storage_area_bundle,
            &mut gatherable_query,
            &mut used_gatherables,
            map,
        );

        let mut entity_commands = commands.entity(entity);
        if let Some(task) = possible_task {
            match task {
                GathererTask::Gather(it) => entity_commands.insert(it),
                GathererTask::Empty(it) => entity_commands.insert(it),
            };
        } else if gatherer.is_automatically_assigned() || manual_assignment.will_reassign() {
            entity_commands.remove::<Gatherer>();
        }
    }
}

#[derive(Debug)]
enum GathererTask {
    Gather(GatherTask),
    Empty(EmptyResourcesTask),
}

fn build_gatherer_task(
    visibility_box: &GridBox,
    resource_inventory: &ResourceInventory,
    storage_area_bundle: Option<(&StorageArea, &GridBody, Entity)>,
    gatherable_query: &mut Query<(&mut Gatherable, &GridBody, &Visibility, Entity)>,
    used_gatherables: &mut Vec<Entity>,
    map: &Map,
) -> Option<GathererTask> {
    let task = if let Some((_, body, entity)) = storage_area_bundle {
        if resource_inventory.total() >= 50 {
            pathfind(map, &visibility_box.center, &body.center_coordinate).map(|path| {
                GathererTask::Empty(EmptyResourcesTask {
                    target: EmptyResourcesTarget {
                        storage_area: entity,
                        path: Path {
                            direction: None,
                            points: path
                                .iter()
                                .map(|point| VisitedPoint::from(*point))
                                .collect(),
                        },
                        done: false,
                    },
                })
            })
        } else {
            None
        }
    } else {
        None
    };

    task.or_else(|| {
        let possible_gatherable_bundle = gatherable_query
            .iter_mut()
            .filter(|(gatherable, _, visibility, entity)| {
                !gatherable.targeted
                    && !used_gatherables.contains(entity)
                    && matches!(visibility, Visibility::Visible | Visibility::Inherited)
            })
            .min_by_key(|(_, body, _, _)| body.center_coordinate.distance(&visibility_box.center));

        if let Some((mut gatherable, body, _, entity)) = possible_gatherable_bundle {
            if body.center_coordinate.distance(&visibility_box.center) <= 1 {
                used_gatherables.push(entity);
                gatherable.targeted = true;
                Some(GathererTask::Gather(GatherTask {
                    target: GatheringTarget {
                        entity: Some(entity),
                        coordinate: body.center_coordinate,
                        path: Path {
                            direction: None,
                            points: Vec::new(),
                        },
                    },
                }))
            } else {
                pathfind(map, &visibility_box.center, &body.center_coordinate).map(|path| {
                    used_gatherables.push(entity);
                    gatherable.targeted = true;
                    GathererTask::Gather(GatherTask {
                        target: GatheringTarget {
                            entity: Some(entity),
                            coordinate: body.center_coordinate,
                            path: Path {
                                direction: None,
                                points: path
                                    .iter()
                                    .map(|point| VisitedPoint::from(*point))
                                    .collect(),
                            },
                        },
                    })
                })
            }
        } else {
            None
        }
    })
}
