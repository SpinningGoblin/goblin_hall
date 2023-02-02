use bevy::prelude::Component;

use crate::components::movement::{Direction, Speed};

#[derive(Debug, Clone, Component, Default)]
pub struct CameraMovement {
    pub direction: Option<Direction>,
    pub speed: Speed,
}

impl CameraMovement {
    pub fn add_direction(&mut self, direction: Direction) {
        let new_direction = if let Some(current) = self.direction {
            current.combine(direction)
        } else {
            Some(direction)
        };

        self.direction = new_direction;
    }
}
