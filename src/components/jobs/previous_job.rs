use bevy::prelude::Component;

use crate::components::tasks::Todo;

use super::Job;

#[derive(Component)]
pub struct PreviousJob {
    pub job: Job,
    pub todo: Todo,
}
