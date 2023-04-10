use bevy::prelude::{Query, Visibility};

use crate::components::{
    jobs::{JobPriority, WithoutJob},
    structures::Mineable,
};

pub fn assign_miner_priority(
    mut query: Query<&mut JobPriority, WithoutJob>,
    structure_query: Query<(&Mineable, &Visibility)>,
) {
    let visible_structures = structure_query
        .into_iter()
        .any(|(_, visibility)| matches!(visibility, Visibility::Visible | Visibility::Inherited));

    // TODO: If I ever have multiple characters, I will need to start
    // assigning less priority to later characters, or figure out a way that
    // the priority for characters who have recently been a miner is more or less.
    let mut miner_assigned = false;
    for mut job_priority in query.iter_mut() {
        job_priority.miner = if miner_assigned {
            false
        } else {
            visible_structures
        };

        if job_priority.miner {
            miner_assigned = true;
        }
    }
}
