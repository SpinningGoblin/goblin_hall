use bevy::prelude::Component;

use crate::components::tasks::Task;

use super::JobType;

#[derive(Component)]
pub struct PreviousJob {
    pub job: JobType,
    pub tasks: Vec<Task>,
}
