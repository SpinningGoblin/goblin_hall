use bevy::prelude::Entity;

use crate::components::movement::Path;

#[derive(Clone, Debug)]
pub struct ExplorationTarget {
    pub entity: Option<Entity>,
    pub path: Path,
}

impl ExplorationTarget {
    pub fn path_incomplete(&self) -> bool {
        self.path.incomplete()
    }
}
