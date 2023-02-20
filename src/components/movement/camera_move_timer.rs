use std::time::Duration;

use bevy::{prelude::Component, time::Timer};

use crate::resources::config::MovementTimer;

#[derive(Component)]
pub struct CameraMoveTimer {
    pub timer: MovementTimer,
}

impl CameraMoveTimer {
    pub fn reset_movement_timer(&mut self) {
        self.timer.0.reset();
    }

    pub fn tick_movement_timer(&mut self, delta: Duration) -> &Timer {
        self.timer.0.tick(delta)
    }
}
