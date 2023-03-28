use crate::components::{
    movement::{ExplorationTarget, Path},
    structures::{MiningTarget, SetupStorageArea},
};

#[derive(Clone)]
pub enum Task {
    Walk(Path),
    Mine(MiningTarget),
    ClearExplorationTarget(ExplorationTarget),
    SetupStorageArea(SetupStorageArea),
}

impl Task {
    pub fn is_complete(&self) -> bool {
        match self {
            Task::Walk(path) => path
                .points
                .iter()
                .all(|visited_point| visited_point.visited),
            Task::Mine(target) => target.entity.is_none(),
            Task::ClearExplorationTarget(target) => target.entity.is_none(),
            Task::SetupStorageArea(setup) => setup.done,
        }
    }
}
