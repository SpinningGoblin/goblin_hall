use bevy::prelude::{Entity, Query, Transform, Visibility};

use crate::{
    components::{
        characters::ResourceInventory,
        jobs::{JobPriority, WithoutJob},
        resources::Gatherable,
        structures::{GridBody, StorageArea},
        Map,
    },
    resources::config::grid::grid_coordinate_from_world,
};

pub fn assign_gatherer_priority(
    mut query: Query<(&mut JobPriority, &ResourceInventory, &Transform), WithoutJob>,
    resource_query: Query<(&Gatherable, &Visibility, &GridBody, Entity)>,
    storage_area_query: Query<&StorageArea>,
    map_query: Query<&Map>,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };

    let has_storage_area = storage_area_query.iter().next().is_some();
    if !has_storage_area {
        return;
    }

    let mut closest_gatherables: Vec<Entity> = Vec::new();
    for (mut job_priority, resource_inventory, transform) in query.iter_mut() {
        if resource_inventory.total() >= 50 {
            job_priority.gatherer.has_full_resource_inventory = true;
        } else {
            let character_coordinate = grid_coordinate_from_world(
                &transform.translation.truncate(),
                map.grid_size,
                map.tile_size,
            );
            let closest = resource_query
                .iter()
                .filter(|(gatherable, visibility, _, entity)| {
                    matches!(visibility, Visibility::Visible | Visibility::Inherited)
                        && !gatherable.targeted
                        && !closest_gatherables.contains(entity)
                })
                .min_by_key(|(_, _, body, _)| {
                    body.center_coordinate.distance(&character_coordinate)
                })
                .map(|(_, _, body, entity)| {
                    (
                        body.center_coordinate.distance(&character_coordinate),
                        entity,
                    )
                });

            if let Some((distance, entity)) = closest {
                job_priority.gatherer.closest_gatherable_distance = Some(distance as u128);
                closest_gatherables.push(entity);
            }
        }
    }
}
