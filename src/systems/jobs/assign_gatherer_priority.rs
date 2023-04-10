use bevy::prelude::{Query, Visibility};

use crate::components::{
    characters::ResourceInventory,
    jobs::{JobPriority, WithoutJob},
    resources::Gatherable,
    structures::StorageArea,
};

pub fn assign_gatherer_priority(
    mut query: Query<(&mut JobPriority, &ResourceInventory), WithoutJob>,
    resource_query: Query<(&Gatherable, &Visibility)>,
    storage_area_query: Query<&StorageArea>,
) {
    let visible_resources = resource_query.into_iter().any(|(gatherable, visibility)| {
        matches!(visibility, Visibility::Visible | Visibility::Inherited) && !gatherable.targeted
    });
    let has_storage_area = storage_area_query.iter().next().is_some();

    // TODO: I should also only do this if there's a place to store the resources, the storage area.
    // Also, I should not assign gatherer if the character's resource inventory is full.
    for (mut job_priority, resource_inventory) in query.iter_mut() {
        job_priority.gatherer =
            (resource_inventory.total() >= 50 || visible_resources) && has_storage_area;
    }
}
