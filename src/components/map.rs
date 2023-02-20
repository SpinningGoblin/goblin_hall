use bevy::prelude::Component;
use tdlg::map::TopDownMap;

#[derive(Component, Debug)]
pub struct Map {
    pub current: TopDownMap,
    pub grid_size: u16,
    pub tile_size: f32,
}
