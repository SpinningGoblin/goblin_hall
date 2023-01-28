use bevy::{prelude::Resource, time::Timer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementTimerConfig {
    pub wait_time: f32,
}

impl MovementTimerConfig {
    pub fn movement_timer(&self) -> MovementTimer {
        MovementTimer(Timer::from_seconds(
            self.wait_time,
            bevy::time::TimerMode::Repeating,
        ))
    }
}

#[derive(Debug, Resource)]
pub struct MovementTimer(pub Timer);
