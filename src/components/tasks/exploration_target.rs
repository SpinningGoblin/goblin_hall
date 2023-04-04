use bevy::prelude::Entity;

#[derive(Clone)]
pub struct ExplorationTarget {
    pub entity: Option<Entity>,
}
