use bevy::prelude::{Commands, Entity, Query, Transform, With, Without};
use strum::IntoEnumIterator;

use crate::{
    components::{
        characters::Character,
        jobs::{ExplorationHistory, Explorer, JobType, PreviousJob},
        movement::{Direction, Path, VisitedPoint},
        structures::GridBody,
        tasks::{ExplorationTarget, Task},
        zones::ExplorationZone,
        GridBox, Map,
    },
    resources::config::grid::grid_coordinate_from_world,
    utils::movement::{find_path, path_to_point},
};

type CharacterWithTransform = (
    &'static Character,
    Entity,
    &'static Transform,
    Option<&'static PreviousJob>,
);

type ExplorerWithoutTask = (With<Explorer>, Without<Task>);

pub fn assign_explorer_task(
    mut commands: Commands,
    query: Query<CharacterWithTransform, ExplorerWithoutTask>,
    explore_history_query: Query<&ExplorationHistory>,
    exploration_zone_query: Query<(&ExplorationZone, &GridBody, Entity)>,
    map_query: Query<&Map>,
) {
    let (Ok(map), Ok(exploration_history)) = (map_query.get_single(), explore_history_query.get_single()) else {
        return;
    };

    let mut used_directions: Vec<Direction> = Vec::new();
    let exploration_zones = exploration_zone_query
        .iter()
        .map(|(_, body, entity)| (body, entity))
        .collect::<Vec<(&GridBody, Entity)>>();
    let mut used_zones: Vec<Entity> = Vec::new();

    for character_bundle in query.iter() {
        let (character, entity, transform, possible_previous_job) = character_bundle;

        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = character.visibility_box(character_coordinate);

        let exploration_zone = exploration_zones
            .iter()
            .min_by_key(|(body, _)| body.center_coordinate.distance(&visibility_box.center));

        let possible_task = build_explore_task(
            &mut used_directions,
            map,
            &visibility_box,
            possible_previous_job,
            exploration_history,
            exploration_zone,
        );

        if let Some((_, entity)) = exploration_zone {
            used_zones.push(*entity);
        }

        if let Some(task) = possible_task {
            commands.entity(entity).insert(task);
        }
    }
}

fn build_explore_task(
    used_directions: &mut Vec<Direction>,
    map: &Map,
    visibility_box: &GridBox,
    possible_previous_job: Option<&PreviousJob>,
    exploration_history: &ExplorationHistory,
    exploration_zone: Option<&(&GridBody, Entity)>,
) -> Option<Task> {
    let task = if let Some((body, entity)) = exploration_zone {
        path_to_point(map, &visibility_box.center, &body.center_coordinate).map(|path| {
            Task::ClearExplorationTarget(ExplorationTarget {
                entity: Some(*entity),
                path: Path {
                    direction: None,
                    points: path
                        .iter()
                        .map(|point| VisitedPoint::from(*point))
                        .collect(),
                },
            })
        })
    } else {
        None
    };

    task.or_else(|| {
        possible_previous_job
            .filter(|previous_job| matches!(previous_job.job, JobType::Explorer))
            .and_then(|previous_job| {
                previous_job.tasks.iter().find_map(|task| match task {
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
                .map(Task::Walk)
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
                Task::Walk(path)
            })
    })
}
