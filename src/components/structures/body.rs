use bevy::prelude::{Component, Vec2};
use tdlg::map::cells::Coordinate;

#[derive(Component)]
pub struct Body {
    pub tile_size: f32,
    pub cell_center: Vec2,
    pub center_coordinate: Coordinate,
    pub underground: bool,
}
