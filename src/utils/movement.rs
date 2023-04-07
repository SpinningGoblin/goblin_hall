use bevy::prelude::{EventWriter, Transform, Vec2};
use tdlg::map::cells::Coordinate;

use crate::{
    components::{movement::Path, Map},
    events::PointVisited,
    resources::config::grid::pathfind,
};

pub fn visit_next_point(
    path: &mut Path,
    transform: &mut Transform,
    event_writer: &mut EventWriter<PointVisited>,
) {
    if let Some(visited_point) = path
        .points
        .iter_mut()
        .find(|visited_point| !visited_point.visited)
    {
        transform.translation.x = visited_point.point.x;
        transform.translation.y = visited_point.point.y;
        visited_point.visited = true;
        event_writer.send(PointVisited {
            point: visited_point.point,
        });
    }
}

pub fn path_to_point(map: &Map, start: &Coordinate, end: &Coordinate) -> Option<Vec<Vec2>> {
    pathfind(map, start, end)
}
