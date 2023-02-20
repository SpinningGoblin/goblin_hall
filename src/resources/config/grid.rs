use bevy::math::Vec2;
use pathfinding::prelude::astar;
use tdlg::map::cells::Coordinate;

use crate::components::Map;

pub fn world_coordinate_from_grid(
    grid_coordinate: &Coordinate,
    map_size: u16,
    tile_size: f32,
) -> Vec2 {
    // conversion formula: tile_size * coordinate - (tile_size * (map_width / 2))
    let x = tile_size * grid_coordinate.x() as f32 - (tile_size * (map_size / 2) as f32);
    let y = tile_size * grid_coordinate.y() as f32 - (tile_size * (map_size / 2) as f32);

    Vec2::new(x, y)
}

pub fn grid_coordinate_from_world(
    world_coordinate: &Vec2,
    map_size: u16,
    tile_size: f32,
) -> Coordinate {
    let x: i32 = ((world_coordinate.x + (tile_size * (map_size / 2) as f32)) / tile_size) as i32;
    let y: i32 = ((world_coordinate.y + (tile_size * (map_size / 2) as f32)) / tile_size) as i32;

    Coordinate::from((x, y))
}

#[allow(dead_code)]
pub fn estimated_distance(
    start: &Coordinate,
    end: &Vec2,
    map_size: u16,
    tile_size: f32,
) -> (Coordinate, u32) {
    let grid_end = grid_coordinate_from_world(end, map_size, tile_size);
    (grid_end, grid_end.distance(start))
}

#[allow(dead_code)]
pub fn pathfind(map: &Map, start: &Coordinate, end: &Coordinate) -> Option<Vec<Vec2>> {
    astar(
        start,
        |c| {
            map.current
                .grid()
                .surrounding_walkable_coordinates(c)
                .into_iter()
                .map(|c| (c, 1))
        },
        |c| c.distance(end) / 3,
        |c| c.distance(end) <= 1,
    )
    .map(|(coords, _)| coords)
    .map(|coords| {
        coords
            .iter()
            .map(|coord| world_coordinate_from_grid(coord, map.grid_size, map.tile_size))
            .collect()
    })
}
