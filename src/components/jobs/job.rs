use bevy::prelude::Component;

#[derive(Component, Clone)]
pub enum JobType {
    Miner,
    Explorer,
    Builder,
}

impl Default for JobType {
    fn default() -> Self {
        Self::Explorer
    }
}

pub trait Job {
    fn job_type(&self) -> JobType;
}

#[derive(Component, Clone, Debug)]
pub struct Miner;

impl Job for Miner {
    fn job_type(&self) -> JobType {
        JobType::Miner
    }
}

#[derive(Component, Clone, Debug)]
pub struct Explorer;

impl Job for Explorer {
    fn job_type(&self) -> JobType {
        JobType::Explorer
    }
}

#[derive(Component, Clone, Debug)]
pub struct Builder;

impl Job for Builder {
    fn job_type(&self) -> JobType {
        JobType::Builder
    }
}
