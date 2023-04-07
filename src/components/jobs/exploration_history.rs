use bevy::prelude::{Component, Vec2};

#[derive(Component, Default)]
pub struct ExplorationHistory {
    pub points: Vec<Vec2>,
}

impl ExplorationHistory {
    pub fn contains(&self, points: &[Vec2]) -> bool {
        points.iter().all(|point| self.points.contains(point))
    }

    pub fn push(&mut self, point: Vec2) {
        if !self.points.contains(&point) {
            self.points.push(point);
        }
    }
}
