use bevy::prelude::Query;

use crate::components::{
    characters::Character,
    jobs::{JobPriority, WithoutJob},
    zones::SetupStorageAreaZone,
};

pub fn assign_builder_priority(
    mut query: Query<(&mut JobPriority, &Character), WithoutJob>,
    setup_storage_zone_query: Query<&SetupStorageAreaZone>,
) {
    let mut count_storage_setup_zones = setup_storage_zone_query
        .iter()
        .filter(|setup_zone| !setup_zone.targeted)
        .collect::<Vec<_>>()
        .len();

    for (mut job_priority, _) in query.iter_mut() {
        if count_storage_setup_zones > 0 {
            job_priority.builder.untargeted_storage_setup = true;
            count_storage_setup_zones -= 1;
        }
    }
}
