use bevy::prelude::{Commands, Entity, Query, Transform, Without};

use crate::{
    components::{
        characters::Character,
        jobs::Job,
        structures::{GridBody, Mineable},
        Map,
    },
    resources::config::grid::grid_coordinate_from_world,
};

pub fn assign_job(
    mut commands: Commands,
    query: Query<(&Character, &Transform, Entity), Without<Job>>,
    structure_query: Query<(&Mineable, &GridBody)>,
    map_query: Query<&Map>,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };

    // I need to identify what needs to be done for each of the characters.
    // This will be based on the current state of the world and what they can see.
    // For now I'm putting all of this into a single system, and will eventually
    // split this out later if I can find a way to do it.
    for character_bundle in query.iter() {
        let (character, transform, entity) = character_bundle;

        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = character.visibility_box(character_coordinate);
        let structures_in_range = structure_query
            .into_iter()
            .filter(|(_, body)| visibility_box.contains(&body.center_coordinate))
            .next()
            .is_some();

        if structures_in_range {
            commands.entity(entity).insert(Job::Explorer);
        } else {
            commands.entity(entity).insert(Job::Miner);
        }
    }
}
