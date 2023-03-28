use bevy::prelude::{info, Commands, Entity, Query, Transform, Vec2, Without};
use strum::IntoEnumIterator;
use tdlg::map::{
    cells::Coordinate,
    layers::{LayerType, StructureType},
};

use crate::{
    components::{
        characters::Character,
        jobs::{ExplorationHistory, Job, PreviousJob},
        movement::{Direction, ExplorationTarget, Path, VisitedPoint},
        structures::{GridBody, Mineable, MiningTarget, SetupStorageArea},
        tasks::{Task, Todo},
        zones::{Zone, ZoneType},
        GridBox, Map,
    },
    resources::config::grid::{grid_coordinate_from_world, pathfind},
};

type JobsComponents = (
    &'static Character,
    &'static Transform,
    Entity,
    &'static Job,
    Option<&'static PreviousJob>,
);

pub fn build_todo(
    mut commands: Commands,
    query: Query<JobsComponents, Without<Todo>>,
    mineable_query: Query<(&Mineable, &GridBody, Entity)>,
    map_query: Query<&Map>,
    explore_history_query: Query<&ExplorationHistory>,
    exploration_zone_query: Query<(&Zone, &GridBody, &ZoneType, Entity)>,
) {
    let (Ok(map), Ok(exploration_history)) = (map_query.get_single(), explore_history_query.get_single()) else {
        return;
    };

    let mut used_directions: Vec<Direction> = Vec::new();
    let exploration_zones = exploration_zone_query
        .iter()
        .map(|(_, body, zone_type, entity)| (body, zone_type, entity))
        .collect::<Vec<(&GridBody, &ZoneType, Entity)>>();
    let mut used_zones: Vec<Entity> = Vec::new();

    for character_bundle in query.iter() {
        info!("Building Todo for character");
        let (character, transform, entity, job, possible_previous_job) = character_bundle;

        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = character.visibility_box(character_coordinate);

        let possible_todo = match job {
            Job::Explorer => {
                let exploration_zone = exploration_zones
                    .iter()
                    .filter(|(_, _, entity)| !used_zones.contains(entity))
                    .min_by_key(|(body, _, _)| {
                        body.center_coordinate.distance(&visibility_box.center)
                    });

                let todo = build_explore_todo(
                    &mut used_directions,
                    map,
                    &visibility_box,
                    possible_previous_job,
                    exploration_history,
                    exploration_zone,
                );

                if let Some((_, _, entity)) = exploration_zone {
                    used_zones.push(*entity);
                }

                todo
            }
            Job::Miner => build_miner_todo(&visibility_box, &mineable_query, map),
        };

        if let Some(todo) = possible_todo {
            commands.entity(entity).insert(todo);
        }
    }
}

fn build_miner_todo(
    visibility_box: &GridBox,
    mineable_query: &Query<(&Mineable, &GridBody, Entity)>,
    map: &Map,
) -> Option<Todo> {
    let mut seen_structures: Vec<(&Mineable, &GridBody, Entity)> = mineable_query
        .iter()
        .filter(|(structure, body, _)| {
            is_wall(&structure.layer_type) && visibility_box.contains(&body.center_coordinate)
        })
        .collect();
    seen_structures
        .sort_by_key(|(_, body, _)| body.center_coordinate.distance(&visibility_box.center));

    seen_structures
        .iter()
        .find_map(|(structure, body, entity)| {
            if body.center_coordinate.distance(&visibility_box.center) <= 1 {
                Some(Todo {
                    tasks: vec![Task::Mine(MiningTarget {
                        entity: Some(*entity),
                        layer_type: Some(structure.layer_type),
                        coordinate: body.center_coordinate,
                    })],
                })
            } else {
                pathfind(map, &visibility_box.center, &body.center_coordinate).map(|path| Todo {
                    tasks: vec![
                        Task::Walk(Path {
                            direction: None,
                            points: path
                                .iter()
                                .map(|point| VisitedPoint::from(*point))
                                .collect(),
                        }),
                        Task::Mine(MiningTarget {
                            entity: Some(*entity),
                            layer_type: Some(structure.layer_type),
                            coordinate: body.center_coordinate,
                        }),
                    ],
                })
            }
        })
}

fn is_wall(layer_type: &LayerType) -> bool {
    matches!(*layer_type, LayerType::Structure(StructureType::Wall))
}

fn build_explore_todo(
    used_directions: &mut Vec<Direction>,
    map: &Map,
    visibility_box: &GridBox,
    possible_previous_job: Option<&PreviousJob>,
    exploration_history: &ExplorationHistory,
    exploration_zone: Option<&(&GridBody, &ZoneType, Entity)>,
) -> Option<Todo> {
    let todo = if let Some((body, zone_type, entity)) = exploration_zone {
        path_to_point(map, &visibility_box.center, &body.center_coordinate).map(|path| {
            let mut tasks = vec![
                Task::Walk(Path {
                    direction: None,
                    points: path
                        .iter()
                        .map(|point| VisitedPoint::from(*point))
                        .collect(),
                }),
                Task::ClearExplorationTarget(ExplorationTarget {
                    entity: Some(*entity),
                }),
            ];

            if matches!(*zone_type, ZoneType::SetupStorageArea) {
                tasks.push(Task::SetupStorageArea(SetupStorageArea {
                    coordinate: body.center_coordinate,
                    done: false,
                }))
            }
            Todo { tasks }
        })
    } else {
        None
    };

    todo.or_else(|| {
        possible_previous_job
            .filter(|previous_job| matches!(previous_job.job, Job::Explorer))
            .and_then(|previous_job| {
                previous_job.todo.tasks.iter().find_map(|task| match task {
                    Task::Walk(path) => path.direction,
                    _ => None,
                })
            })
            .and_then(|direction| {
                find_path(
                    map,
                    &visibility_box.farthest_coordinate_at_direction(&direction),
                    &visibility_box.center,
                    Some(direction),
                    exploration_history,
                )
                .map(|path| Todo {
                    tasks: vec![Task::Walk(path)],
                })
            })
    })
    .or_else(|| {
        Direction::iter()
            .filter(|direction| !used_directions.contains(direction))
            .find_map(|direction| {
                find_path(
                    map,
                    &visibility_box.farthest_coordinate_at_direction(&direction),
                    &visibility_box.center,
                    Some(direction),
                    exploration_history,
                )
            })
            .map(|path| {
                if let Some(direction) = path.direction {
                    used_directions.push(direction);
                }
                Todo {
                    tasks: vec![Task::Walk(path)],
                }
            })
    })
}

fn path_to_point(map: &Map, start: &Coordinate, end: &Coordinate) -> Option<Vec<Vec2>> {
    pathfind(map, start, end)
}

fn find_path(
    map: &Map,
    coordinate: &Coordinate,
    character_coordinate: &Coordinate,
    direction: Option<Direction>,
    exploration_history: &ExplorationHistory,
) -> Option<Path> {
    if !map.is_coordinate_walkable(coordinate) {
        return None;
    }

    path_to_point(map, character_coordinate, coordinate)
        .filter(|points| !exploration_history.contains(points))
        .map(|path| Path {
            direction,
            points: path
                .iter()
                .map(|point| VisitedPoint::from(*point))
                .collect(),
        })
}
