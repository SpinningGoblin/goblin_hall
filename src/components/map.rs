use bevy::prelude::Component;
use tdlg::map::{cells::Coordinate, TopDownMap};

#[derive(Component, Debug)]
pub struct Map {
    pub current: TopDownMap,
    pub grid_size: u16,
    pub tile_size: f32,
}

impl Map {
    pub fn is_coordinate_walkable(&self, coordinate: &Coordinate) -> bool {
        self.current
            .grid()
            .cell(coordinate)
            .map(|cell| cell.is_walkable())
            .unwrap_or_default()
    }
}
