use bevy::prelude::{Commands, Entity, Query};

use crate::components::{
    jobs::{Job, PreviousJob},
    tasks::Todo,
};

pub fn remove_todo(mut commands: Commands, query: Query<(Entity, &Todo, Option<&Job>)>) {
    for todo_bundle in query.iter() {
        let (entity, todo, possible_job) = todo_bundle;

        if todo.tasks.iter().all(|task| task.is_complete()) {
            commands.entity(entity).remove::<Todo>();

            if let Some(job) = possible_job {
                commands.entity(entity).remove::<Job>().insert(PreviousJob {
                    job: job.clone(),
                    todo: todo.clone(),
                });
            }
        }
    }
}
