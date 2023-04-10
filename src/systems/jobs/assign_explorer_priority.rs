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
    let mut count_exploration_zones = exploration_zone_query
        .iter()
        .filter(|explore_zone| !explore_zone.targeted)
        .collect::<Vec<_>>()
        .len();

    for (mut priority, _) in query.iter_mut() {
        if count_exploration_zones > 0 {
            priority.explorer.untargeted_zone = true;
            count_exploration_zones -= 1;
        }
    }
}
