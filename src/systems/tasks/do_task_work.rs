use bevy::prelude::{info, Commands, Query, Transform};

use crate::components::{
    characters::Character,
    jobs::ExplorationHistory,
    tasks::{Task, Todo},
    Map, World,
};

pub fn do_task_work(
    mut commands: Commands,
    mut query: Query<(&Character, &mut Transform, &mut Todo)>,
    mut exploration_history_query: Query<&mut ExplorationHistory>,
    world_query: Query<&World>,
    mut map_query: Query<&mut Map>,
) {
    if world_query.is_empty() || exploration_history_query.is_empty() || map_query.is_empty() {
        return;
    }

    let world = world_query.single();
    if !world.tick_just_finished {
        return;
    }

    let mut map = map_query.single_mut();

    let mut exploration_history = exploration_history_query.single_mut();
    for character_bundle in query.iter_mut() {
        let (_, mut transform, mut todo) = character_bundle;

        if let Some(current_task) = todo.tasks.iter_mut().find(|task| !task.is_complete()) {
            match current_task {
                Task::Walk(ref mut path) => {
                    info!("Walking");
                    if let Some(visited_point) = path
                        .points
                        .iter_mut()
                        .find(|visited_point| !visited_point.visited)
                    {
                        transform.translation.x = visited_point.point.x;
                        transform.translation.y = visited_point.point.y;
                        visited_point.visited = true;
                        exploration_history.points.push(visited_point.point);
                    }
                }
                Task::Mine(mining_target) => {
                    if let Some(entity) = mining_target.entity {
                        commands.entity(entity).despawn();
                        mining_target.entity = None;
                        if let Some(layer) = &mining_target.layer_type {
                            map.current
                                .grid_mut()
                                .remove_layer(&mining_target.coordinate, *layer);
                        }
                    }
                }
                Task::ClearExplorationTarget(exploration_target) => {
                    if let Some(entity) = exploration_target.entity {
                        commands.entity(entity).despawn();
                        exploration_target.entity = None;
                    }
                }
            };
        }
    }
}
