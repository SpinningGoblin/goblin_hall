use bevy::math::Vec2;
use tdlg::map::cells::Coordinate;

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

#[allow(dead_code)]
pub fn grid_coordinate_from_world(
    world_coordinate: &Vec2,
    map_size: u16,
    tile_size: f32,
) -> Coordinate {
    let x: i32 = ((world_coordinate.x + (tile_size * (map_size / 2) as f32)) / tile_size) as i32;
    let y: i32 = ((world_coordinate.y + (tile_size * (map_size / 2) as f32)) / tile_size) as i32;

    Coordinate::from((x, y))
}
