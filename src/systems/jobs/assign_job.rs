use bevy::prelude::{info, Commands, Entity, Query, Transform, Without};

use crate::{
    components::{
        characters::Character,
        jobs::{Builder, Explorer, Miner},
        structures::{GridBody, Mineable},
        zones::{ExplorationZone, SetupStorageAreaZone},
        Map,
    },
    resources::config::grid::grid_coordinate_from_world,
};

type WithoutJob = (Without<Miner>, Without<Explorer>, Without<Builder>);

pub fn assign_job(
    mut commands: Commands,
    query: Query<(&Character, &Transform, Entity), WithoutJob>,
    structure_query: Query<(&Mineable, &GridBody)>,
    exploration_zone_query: Query<&ExplorationZone>,
    setup_storage_zone_query: Query<&SetupStorageAreaZone>,
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
            .any(|(_, body)| visibility_box.contains(&body.center_coordinate));

        let has_exploration_zone = exploration_zone_query.iter().next().is_some();
        let has_setup_storage_zone = setup_storage_zone_query.iter().next().is_some();

        let mut entity_commands = commands.entity(entity);

        if has_setup_storage_zone {
            info!("Making builder");
            entity_commands.insert(Builder);
        } else if has_exploration_zone {
            info!("Making Explorer from exploration zone");
            entity_commands.insert(Explorer);
        } else if structures_in_range {
            info!("Making miner");
            entity_commands.insert(Miner);
        } else {
            info!("Making default Explorer job");
            entity_commands.insert(Explorer);
        }
    }
}
