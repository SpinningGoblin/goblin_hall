use bevy::prelude::Entity;
use tdlg::map::cells::Coordinate;

use crate::components::movement::Path;

#[derive(Clone, Debug)]
pub struct GatheringTarget {
    pub entity: Option<Entity>,
    pub coordinate: Coordinate,
    pub path: Path,
}

impl GatheringTarget {
    pub fn path_incomplete(&self) -> bool {
        self.path.incomplete()
    }
}
