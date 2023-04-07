use bevy::prelude::Entity;
use tdlg::map::cells::Coordinate;

use crate::components::movement::Path;

#[derive(Clone, Debug)]
pub struct SetupStorageArea {
    pub done: bool,
    pub entity: Option<Entity>,
    pub coordinate: Coordinate,
    pub path: Path,
}

impl SetupStorageArea {
    pub fn path_incomplete(&self) -> bool {
        self.path.incomplete()
    }
}
