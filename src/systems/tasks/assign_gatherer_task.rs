use bevy::prelude::{Commands, Entity, Query, Transform, With};

use crate::{
    components::{
        characters::{Character, ResourceInventory},
        jobs::Gatherer,
        movement::{Path, VisitedPoint},
        resources::Resource,
        structures::{GridBody, StorageArea},
        tasks::{
            EmptyResourcesTarget, EmptyResourcesTask, GatherTask, GatheringTarget, WithoutTask,
        },
        GridBox, Map,
    },
    resources::config::grid::{grid_coordinate_from_world, pathfind},
};

type CharacterTransform = (
    &'static Character,
    Entity,
    &'static Transform,
    &'static ResourceInventory,
);
type GathererWithoutTask = (With<Gatherer>, WithoutTask);

pub fn assign_gatherer_task(
    mut commands: Commands,
    query: Query<CharacterTransform, GathererWithoutTask>,
    resource_query: Query<(&Resource, &GridBody, Entity)>,
    storage_area_query: Query<(&StorageArea, &GridBody, Entity)>,
    map_query: Query<&Map>,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };

    for character_bundle in query.iter() {
        let (character, entity, transform, resource_inventory) = character_bundle;

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
            &resource_query,
            map,
        );

        if let Some(task) = possible_task {
            match task {
                GathererTask::Gather(it) => commands.entity(entity).insert(it),
                GathererTask::Empty(it) => commands.entity(entity).insert(it),
            };
        } else {
            commands.entity(entity).remove::<Gatherer>();
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
    mineable_query: &Query<(&Resource, &GridBody, Entity)>,
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
        let mut seen_resources: Vec<(&Resource, &GridBody, Entity)> = mineable_query
            .iter()
            .filter(|(_, body, _)| visibility_box.contains(&body.center_coordinate))
            .collect();
        seen_resources
            .sort_by_key(|(_, body, _)| body.center_coordinate.distance(&visibility_box.center));

        seen_resources.iter().find_map(|(_, body, entity)| {
            if body.center_coordinate.distance(&visibility_box.center) <= 1 {
                Some(GathererTask::Gather(GatherTask {
                    target: GatheringTarget {
                        entity: Some(*entity),
                        coordinate: body.center_coordinate,
                        path: Path {
                            direction: None,
                            points: Vec::new(),
                        },
                    },
                }))
            } else {
                pathfind(map, &visibility_box.center, &body.center_coordinate).map(|path| {
                    GathererTask::Gather(GatherTask {
                        target: GatheringTarget {
                            entity: Some(*entity),
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
        })
    })
}
