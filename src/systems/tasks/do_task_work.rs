use bevy::prelude::{Query, Transform};

use crate::components::{
    characters::Character,
    tasks::{Task, Todo},
    World,
};

pub fn do_task_work(
    mut query: Query<(&Character, &mut Transform, &mut Todo)>,
    world_query: Query<&World>,
) {
    if world_query.is_empty() {
        return;
    }

    let world = world_query.single();
    if !world.tick_just_finished {
        return;
    }

    for character_bundle in query.iter_mut() {
        let (_, mut transform, mut todo) = character_bundle;

        if let Some(current_task) = todo.tasks.last_mut() {
            match current_task {
                Task::Walk(ref mut path) => {
                    if let Some(point) = path.points.pop() {
                        transform.translation.x = point.x;
                        transform.translation.y = point.y;
                    }
                }
            };
        }

        todo.tasks.retain(|task| !task.is_complete());
    }
}
