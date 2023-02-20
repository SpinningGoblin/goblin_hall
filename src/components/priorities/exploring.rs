use bevy::prelude::Component;

use super::{Priority, PriorityLevel};

#[derive(Component)]
pub struct Exploring {
}

impl Priority for Exploring {
    fn priority_level(&self) -> PriorityLevel {
        PriorityLevel::Low
    }
}
