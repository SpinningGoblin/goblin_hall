use bevy::prelude::Vec2;

pub fn x_axis_speed(tile_size: f32, modifier: f32) -> Vec2 {
    Vec2::new(tile_size * modifier, 0.0)
}

pub fn y_axis_speed(tile_size: f32, modifier: f32) -> Vec2 {
    Vec2::new(0.0, tile_size * modifier)
}
