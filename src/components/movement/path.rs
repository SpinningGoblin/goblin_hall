use bevy::prelude::Vec2;

use super::Direction;

#[derive(Clone, Debug)]
pub struct Path {
    pub direction: Option<Direction>,
    pub points: Vec<VisitedPoint>,
}

impl Path {
    pub fn incomplete(&self) -> bool {
        self.points
            .iter()
            .any(|visited_point| !visited_point.visited)
    }
}

#[derive(Clone, Debug)]
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
