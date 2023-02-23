use bevy::prelude::{Commands, Entity, Query};

use crate::components::{jobs::Job, tasks::Todo};

pub fn remove_todo(mut commands: Commands, query: Query<(Entity, &Todo, Option<&Job>)>) {
    for todo_bundle in query.iter() {
        let (entity, todo, job) = todo_bundle;

        if todo.tasks.is_empty() {
            commands.entity(entity).remove::<Todo>();

            if job.is_some() {
                commands.entity(entity).remove::<Job>();
            }
        }
    }
}
