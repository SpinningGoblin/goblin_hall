use bevy::prelude::Vec2;

use super::Direction;

pub struct Path {
    pub direction: Direction,
    pub points: Vec<Vec2>,
}
