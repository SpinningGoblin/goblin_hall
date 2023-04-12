use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct World {
    pub tick_just_finished: bool,
}

#[derive(Clone, Component, Copy, Debug)]
pub enum WorldTickCalculation {
    Paused,
    Running,
}

impl WorldTickCalculation {
    pub fn flip(&self) -> Self {
        match self {
            WorldTickCalculation::Paused => WorldTickCalculation::Running,
            WorldTickCalculation::Running => WorldTickCalculation::Paused,
        }
    }
}
