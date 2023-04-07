use bevy::prelude::Component;

use crate::components::{
    movement::Path,
    tasks::{ExplorationTarget, MiningTarget, SetupStorageArea},
};

#[derive(Clone, Component, Debug)]
pub struct WalkTask {
    pub path: Path,
}

impl WalkTask {
    pub fn is_complete(&self) -> bool {
        !self.path.incomplete()
    }
}

#[derive(Clone, Component, Debug)]
pub struct MineTask {
    pub target: MiningTarget,
}

impl MineTask {
    pub fn is_complete(&self) -> bool {
        !self.target.path.incomplete() && self.target.entity.is_none()
    }
}

#[derive(Clone, Component, Debug)]
pub struct ClearExplorationTargetTask {
    pub target: ExplorationTarget,
}

impl ClearExplorationTargetTask {
    pub fn is_complete(&self) -> bool {
        !self.target.path.incomplete() && self.target.entity.is_none()
    }
}

#[derive(Clone, Component, Debug)]
pub struct SetupStorageAreaTask {
    pub setup_area: SetupStorageArea,
}

impl SetupStorageAreaTask {
    pub fn is_complete(&self) -> bool {
        self.setup_area.done
    }
}
