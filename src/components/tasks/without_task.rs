use bevy::prelude::Without;

use super::{ClearExplorationTargetTask, MineTask, SetupStorageAreaTask, WalkTask};

pub type WithoutTask = (
    Without<WalkTask>,
    Without<MineTask>,
    Without<SetupStorageAreaTask>,
    Without<ClearExplorationTargetTask>,
);
