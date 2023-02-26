use bevy::prelude::Component;

use super::Task;

#[derive(Component, Clone)]
pub struct Todo {
    pub tasks: Vec<Task>,
}
