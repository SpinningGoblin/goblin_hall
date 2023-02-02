use serde::{Deserialize, Serialize};

use super::{MovementTimer, MovementTimerConfig};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MovementConfig {
    pub timer: MovementTimerConfig,
}

impl MovementConfig {
    pub fn movement_timer(&self) -> MovementTimer {
        self.timer.movement_timer()
    }
}
