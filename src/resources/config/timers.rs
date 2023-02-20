use bevy::{prelude::Resource, time::Timer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementTimerConfig {
    pub wait_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTickTimerConfig {
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

impl WorldTickTimerConfig {
    pub fn timer(&self) -> WorldTickTimer {
        WorldTickTimer(Timer::from_seconds(
            self.wait_time,
            bevy::time::TimerMode::Repeating,
        ))
    }
}

#[derive(Debug, Resource)]
pub struct MovementTimer(pub Timer);

#[derive(Debug, Resource)]
pub struct WorldTickTimer(pub Timer);
