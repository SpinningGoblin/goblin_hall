use bevy::prelude::{info, Commands, Entity, Query};

use crate::components::{
    characters::Character,
    jobs::{
        AssignmentMode, Builder, Explorer, Gatherer, GlobalAssignmentMode, JobPriority, JobType,
        ManualAssignment, Miner, WithoutJob,
    },
};

pub fn assign_job(
    mut commands: Commands,
    mut query: Query<(&Character, &mut JobPriority, Entity, &mut ManualAssignment), WithoutJob>,
    assignment_type_query: Query<&GlobalAssignmentMode>,
) {
    let Ok(global_assignment) = assignment_type_query.get_single() else {
        return;
    };

    // I need to identify what needs to be done for each of the characters.
    // This will be based on the current state of the world and what they can see.
    // For now I'm putting all of this into a single system, and will eventually
    // split this out later if I can find a way to do it.
    for character_bundle in query.iter_mut() {
        let (_, mut job_priority, entity, mut manual_assignment) = character_bundle;

        // Only assign jobs if we're in automatic mode.
        if matches!(global_assignment.mode, AssignmentMode::Automatic)
            || manual_assignment.will_reassign()
        {
            let mut entity_commands = commands.entity(entity);
            let manual_type = manual_assignment
                .job
                .clone()
                .map(|job_type| (job_type, AssignmentMode::Manual));

            let top_priority = manual_type
                .unwrap_or_else(|| (job_priority.top_priority(), AssignmentMode::Automatic));

            manual_assignment.reset();

            info!("{:?} {:?}", &top_priority, entity);

            match top_priority {
                (JobType::Miner, mode) => entity_commands.insert(Miner { assigned_by: mode }),
                (JobType::Explorer, mode) => entity_commands.insert(Explorer { assigned_by: mode }),
                (JobType::Builder, mode) => entity_commands.insert(Builder { assigned_by: mode }),
                (JobType::Gatherer, mode) => entity_commands.insert(Gatherer { assigned_by: mode }),
            };
        }

        // Always reset the job priority so that the next go-round we're good to try again.
        job_priority.reset();
    }
}
