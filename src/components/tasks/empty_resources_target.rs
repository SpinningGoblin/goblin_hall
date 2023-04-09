use bevy::prelude::Entity;

use crate::components::movement::Path;

#[derive(Clone, Debug)]
pub struct EmptyResourcesTarget {
    pub path: Path,
    pub storage_area: Entity,
    pub done: bool,
}

impl EmptyResourcesTarget {
    pub fn path_incomplete(&self) -> bool {
        self.path.incomplete()
    }
}
