use bevy::prelude::{Transform, Vec2};
use tdlg::map::cells::Coordinate;

use crate::{
    components::{
        jobs::ExplorationHistory,
        movement::{Direction, Path, VisitedPoint},
        Map,
    },
    resources::config::grid::pathfind,
};

pub fn visit_next_point(
    path: &mut Path,
    transform: &mut Transform,
    exploration_history: &mut ExplorationHistory,
) {
    if let Some(visited_point) = path
        .points
        .iter_mut()
        .find(|visited_point| !visited_point.visited)
    {
        transform.translation.x = visited_point.point.x;
        transform.translation.y = visited_point.point.y;
        visited_point.visited = true;
        exploration_history.push(visited_point.point);
    }
}

pub fn path_to_point(map: &Map, start: &Coordinate, end: &Coordinate) -> Option<Vec<Vec2>> {
    pathfind(map, start, end)
}

pub fn find_path(
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
