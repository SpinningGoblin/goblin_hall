use bevy::prelude::Vec2;

use super::Direction;

#[derive(Clone)]
pub struct Path {
    pub direction: Option<Direction>,
    pub points: Vec<VisitedPoint>,
}

#[derive(Clone)]
pub struct VisitedPoint {
    pub visited: bool,
    pub point: Vec2,
}

impl From<Vec2> for VisitedPoint {
    fn from(point: Vec2) -> Self {
        Self {
            visited: false,
            point,
        }
    }
}
