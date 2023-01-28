use serde::{Deserialize, Serialize};

use super::{MovementTimer, MovementTimerConfig};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MovementConfig {
    pub timer: MovementTimerConfig,
    pub speed: SpeedConfig,
}

impl MovementConfig {
    pub fn movement_timer(&self) -> MovementTimer {
        self.timer.movement_timer()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeedConfig {
    pub camera_modifier: f32,
}
