use bevy::prelude::Query;

use crate::components::{
    characters::Character,
    jobs::{JobPriority, WithoutJob},
    zones::ExplorationZone,
};

pub fn assign_explorer_priority(
    mut query: Query<(&mut JobPriority, &Character), WithoutJob>,
    exploration_zone_query: Query<&ExplorationZone>,
) {
    let has_exploration_zone = exploration_zone_query.iter().next().is_some();

    for (mut priority, _) in query.iter_mut() {
        priority.explorer = has_exploration_zone;
    }
}
