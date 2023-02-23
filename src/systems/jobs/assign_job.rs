use bevy::prelude::{info, Commands, Entity, Query, Transform, Without};

use crate::{
    components::{
        characters::Character,
        jobs::Job,
        structures::{Body, Structure},
        GridBox, Map, World,
    },
    resources::config::grid::grid_coordinate_from_world,
};

pub fn assign_job(
    mut commands: Commands,
    query: Query<(&Character, &Transform, Entity), Without<Job>>,
    structure_query: Query<(&Structure, &Body)>,
    map_query: Query<&Map>,
    world_query: Query<&World>,
) {
    if world_query.is_empty() || map_query.is_empty() {
        return;
    }

    let world = world_query.single();
    if !world.tick_just_finished {
        return;
    }

    let map = map_query.single();

    // I need to identify what needs to be done for each of the characters.
    // This will be based on the current state of the world and what they can see.
    // For now I'm putting all of this into a single system, and will eventually
    // split this out later if I can find a way to do it.
    for character_bundle in query.iter() {
        let (_, transform, entity) = character_bundle;

        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = GridBox {
            center: character_coordinate,
            size: 10,
        };

        let structures_in_range = structure_query
            .iter()
            .filter(|(_, body)| visibility_box.contains(&body.center_coordinate))
            .collect::<Vec<(&Structure, &Body)>>();

        if structures_in_range.is_empty() {
            info!("Making the character an explorer");
            commands.entity(entity).insert(Job::Explorer);
        } else {
            info!("Making the character a miner");
            commands.entity(entity).insert(Job::Miner);
        }
    }
}
