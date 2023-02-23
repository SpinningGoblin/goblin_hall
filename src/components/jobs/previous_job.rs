use bevy::prelude::Component;

use super::Job;

#[derive(Component)]
pub struct PreviousJob(pub Job);
