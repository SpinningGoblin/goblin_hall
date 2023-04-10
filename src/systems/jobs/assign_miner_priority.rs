use bevy::prelude::{Entity, Query, Transform, Visibility};

use crate::{
    components::{
        jobs::{JobPriority, WithoutJob},
        structures::{GridBody, Mineable},
        Map,
    },
    resources::config::grid::grid_coordinate_from_world,
};

pub fn assign_miner_priority(
    mut query: Query<(&mut JobPriority, &Transform), WithoutJob>,
    mineable_query: Query<(&Mineable, &Visibility, &GridBody, Entity)>,
    map_query: Query<&Map>,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };

    let mut closest_mineables: Vec<Entity> = Vec::new();
    for (mut job_priority, transform) in query.iter_mut() {
        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );
        let closest = mineable_query
            .iter()
            .filter(|(mineable, visibility, _, entity)| {
                matches!(visibility, Visibility::Visible | Visibility::Inherited)
                    && !mineable.targeted
                    && !closest_mineables.contains(entity)
            })
            .min_by_key(|(_, _, body, _)| body.center_coordinate.distance(&character_coordinate))
            .map(|(_, _, body, entity)| {
                (
                    body.center_coordinate.distance(&character_coordinate),
                    entity,
                )
            });

        if let Some((distance, entity)) = closest {
            job_priority.miner.closest_mineable_distance = Some(distance as u128);
            closest_mineables.push(entity);
        }
    }
}
