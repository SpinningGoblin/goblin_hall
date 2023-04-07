use bevy::prelude::{Commands, Entity, Query, Transform, With};
use strum::IntoEnumIterator;
use tdlg::map::cells::Coordinate;

use crate::{
    components::{
        characters::Character,
        jobs::{ExplorationHistory, Explorer, PreviousExplorations},
        movement::{Direction, Path, VisitedPoint},
        structures::GridBody,
        tasks::{ClearExplorationTargetTask, ExplorationTarget, WalkTask, WithoutTask},
        zones::ExplorationZone,
        GridBox, Map,
    },
    resources::config::grid::grid_coordinate_from_world,
    utils::movement::path_to_point,
};

type CharacterWithTransform = (
    &'static Character,
    Entity,
    &'static Transform,
    &'static PreviousExplorations,
);

type ExplorerWithoutTask = (With<Explorer>, WithoutTask);

enum ExplorerTask {
    Walk(WalkTask),
    ClearExploration(ClearExplorationTargetTask),
}

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
        let (character, entity, transform, previous_explorations) = character_bundle;

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
            previous_explorations,
            exploration_history,
            exploration_zone,
        );

        if let Some((_, entity)) = exploration_zone {
            used_zones.push(*entity);
        }

        if let Some(task) = possible_task {
            match task {
                ExplorerTask::Walk(it) => {
                    commands.entity(entity).insert(it);
                }
                ExplorerTask::ClearExploration(it) => {
                    commands.entity(entity).insert(it);
                }
            };
        }
    }
}

pub fn find_exploration_path(
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

fn build_explore_task(
    used_directions: &mut Vec<Direction>,
    map: &Map,
    visibility_box: &GridBox,
    previous_explorations: &PreviousExplorations,
    exploration_history: &ExplorationHistory,
    exploration_zone: Option<&(&GridBody, Entity)>,
) -> Option<ExplorerTask> {
    let task = if let Some((body, entity)) = exploration_zone {
        path_to_point(map, &visibility_box.center, &body.center_coordinate).map(|path| {
            ExplorerTask::ClearExploration(ClearExplorationTargetTask {
                target: ExplorationTarget {
                    entity: Some(*entity),
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
    } else {
        None
    };

    task.or_else(|| {
        previous_explorations.direction.and_then(|direction| {
            find_exploration_path(
                map,
                &visibility_box.farthest_coordinate_at_direction(&direction),
                &visibility_box.center,
                Some(direction),
                exploration_history,
            )
            .map(|path| ExplorerTask::Walk(WalkTask { path }))
        })
    })
    .or_else(|| {
        Direction::iter()
            .filter(|direction| !used_directions.contains(direction))
            .find_map(|direction| {
                find_exploration_path(
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
                ExplorerTask::Walk(WalkTask { path })
            })
    })
}
