use bevy::prelude::Component;

use super::AssignmentMode;

#[derive(Component, Clone, Debug)]
pub struct GlobalAssignmentMode {
    pub mode: AssignmentMode,
}

impl GlobalAssignmentMode {
    pub fn manual() -> Self {
        Self {
            mode: AssignmentMode::Manual,
        }
    }
}
