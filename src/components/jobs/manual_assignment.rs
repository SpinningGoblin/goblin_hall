use bevy::prelude::Component;

use super::JobType;

#[derive(Clone, Component, Debug, Default)]
pub struct ManualAssignment {
    pub job: Option<JobType>,
    pub unassign_automatic: bool,
}

impl ManualAssignment {
    pub fn reset(&mut self) {
        self.job = None;
        self.unassign_automatic = false;
    }

    pub fn will_reassign(&self) -> bool {
        self.job.is_some() || self.unassign_automatic
    }
}
