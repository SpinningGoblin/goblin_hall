use bevy::prelude::Without;

use super::{
    ClearExplorationTargetTask, EmptyResourcesTask, GatherTask, MineTask, SetupStorageAreaTask,
    WalkTask,
};

pub type WithoutTask = (
    Without<WalkTask>,
    Without<MineTask>,
    Without<SetupStorageAreaTask>,
    Without<ClearExplorationTargetTask>,
    Without<GatherTask>,
    Without<EmptyResourcesTask>,
);
