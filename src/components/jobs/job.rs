use bevy::prelude::Component;

use super::AssignmentMode;

#[derive(Component, Clone, Debug)]
pub enum JobType {
    Miner,
    Explorer,
    Builder,
    Gatherer,
}

impl Default for JobType {
    fn default() -> Self {
        Self::Explorer
    }
}

pub trait Job {
    fn job_type(&self) -> JobType;
    fn is_automatically_assigned(&self) -> bool;
}

#[derive(Component, Clone, Debug)]
pub struct Gatherer {
    pub assigned_by: AssignmentMode,
}

impl Job for Gatherer {
    fn job_type(&self) -> JobType {
        JobType::Gatherer
    }

    fn is_automatically_assigned(&self) -> bool {
        matches!(self.assigned_by, AssignmentMode::Automatic)
    }
}

#[derive(Component, Clone, Debug)]
pub struct Miner {
    pub assigned_by: AssignmentMode,
}

impl Job for Miner {
    fn job_type(&self) -> JobType {
        JobType::Miner
    }

    fn is_automatically_assigned(&self) -> bool {
        matches!(self.assigned_by, AssignmentMode::Automatic)
    }
}

#[derive(Component, Clone, Debug)]
pub struct Explorer {
    pub assigned_by: AssignmentMode,
}

impl Job for Explorer {
    fn job_type(&self) -> JobType {
        JobType::Explorer
    }

    fn is_automatically_assigned(&self) -> bool {
        matches!(self.assigned_by, AssignmentMode::Automatic)
    }
}

#[derive(Component, Clone, Debug)]
pub struct Builder {
    pub assigned_by: AssignmentMode,
}

impl Job for Builder {
    fn job_type(&self) -> JobType {
        JobType::Builder
    }

    fn is_automatically_assigned(&self) -> bool {
        matches!(self.assigned_by, AssignmentMode::Automatic)
    }
}
