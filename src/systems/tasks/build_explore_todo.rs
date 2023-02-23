use bevy::prelude::{Commands, Entity, Query, Transform, Without};
use strum::IntoEnumIterator;
use tdlg::map::cells::Coordinate;

use crate::{
    components::{
        characters::Character,
        jobs::Job,
        movement::{Direction, Path},
        tasks::{Task, Todo},
        GridBox, Map, World,
    },
    resources::config::grid::{grid_coordinate_from_world, pathfind},
};

pub fn build_explore_todo(
    mut commands: Commands,
    query: Query<(&Character, &Transform, Entity, &Job), Without<Todo>>,
    map_query: Query<&Map>,
    world_query: Query<&World>,
) {
    if map_query.is_empty() || world_query.is_empty() {
        return;
    }

    let world = world_query.single();
    if !world.tick_just_finished {
        return;
    }

    let map = map_query.single();

    let mut used_directions: Vec<Direction> = Vec::new();

    for character_bundle in query
        .iter()
        .filter(|(_, _, _, job)| matches!(job, Job::Explorer))
    {
        let (_, transform, entity, _) = character_bundle;

        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = GridBox {
            center: character_coordinate,
            size: 10,
        };

        if let Some(path) = Direction::iter()
            .filter(|direction| !used_directions.contains(direction))
            .find_map(|direction| {
                find_path(
                    map,
                    &visibility_box.farthest_coordinate_at_direction(&direction),
                    &character_coordinate,
                    direction,
                )
            })
        {
            used_directions.push(path.direction);
            commands.entity(entity).insert(Todo {
                tasks: vec![Task::Walk(path)],
            });
        }
    }
}

fn find_path(
    map: &Map,
    coordinate: &Coordinate,
    character_coordinate: &Coordinate,
    direction: Direction,
) -> Option<Path> {
    if !map.is_coordinate_walkable(coordinate) {
        return None;
    }

    if let Some(mut path) = pathfind(map, character_coordinate, coordinate) {
        path.reverse();
        Some(Path {
            direction,
            points: path,
        })
    } else {
        None
    }
}
