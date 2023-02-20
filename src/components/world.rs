use std::time::Duration;

use bevy::{prelude::Component, time::Timer};

use crate::resources::config::WorldTickTimer;

#[derive(Component)]
pub struct WorldTimer {
    pub timer: WorldTickTimer,
}

impl WorldTimer {
    pub fn tick(&mut self, delta: Duration) -> &Timer {
        self.timer.0.tick(delta)
    }

    pub fn just_finished(&self) -> bool {
        self.timer.0.just_finished()
    }
}

#[derive(Component, Default)]
pub struct World {
    pub tick_just_finished: bool,
}
