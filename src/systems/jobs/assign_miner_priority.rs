use bevy::prelude::{Query, Transform};

use crate::{
    components::{
        characters::Character,
        jobs::{JobPriority, WithoutJob},
        structures::{GridBody, Mineable},
        Map,
    },
    resources::config::grid::grid_coordinate_from_world,
};

pub fn assign_miner_priority(
    mut query: Query<(&mut JobPriority, &Transform, &Character), WithoutJob>,
    structure_query: Query<(&Mineable, &GridBody)>,
    map_query: Query<&Map>,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };

    for character_bundle in query.iter_mut() {
        let (mut job_priority, transform, character) = character_bundle;

        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = character.visibility_box(character_coordinate);
        let structures_in_range = structure_query
            .into_iter()
            .any(|(_, body)| visibility_box.contains(&body.center_coordinate));

        job_priority.miner = structures_in_range;
    }
}
