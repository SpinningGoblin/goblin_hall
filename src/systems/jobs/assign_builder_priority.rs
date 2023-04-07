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
    let has_setup_storage_zone = setup_storage_zone_query.iter().next().is_some();

    for (mut job_priority, _) in query.iter_mut() {
        job_priority.builder = has_setup_storage_zone;
    }
}
