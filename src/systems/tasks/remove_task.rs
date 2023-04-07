use bevy::prelude::{Commands, Entity, Query};

use crate::components::{
    jobs::{Explorer, Job, Miner, PreviousJob},
    tasks::Task,
};

pub fn remove_task(
    mut commands: Commands,
    query: Query<(Entity, &Task, Option<&Miner>, Option<&Explorer>)>,
) {
    for todo_bundle in query.iter() {
        let (entity, task, possible_miner, possible_explorer) = todo_bundle;

        if task.is_complete() {
            let possible_job_type = possible_miner
                .map(|miner| miner.job_type())
                .or_else(|| possible_explorer.map(|explorer| explorer.job_type()));

            let mut entity_commands = commands.entity(entity);
            entity_commands.remove::<Task>();

            if let Some(job_type) = possible_job_type {
                entity_commands
                    .remove::<Miner>()
                    .remove::<Explorer>()
                    .insert(PreviousJob {
                        job: job_type,
                        tasks: vec![task.clone()],
                    });
            }
        }
    }
}
