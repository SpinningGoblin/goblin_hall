use bevy::prelude::Component;

use super::Task;

#[derive(Component)]
pub struct Todo {
    pub tasks: Vec<Task>,
}
