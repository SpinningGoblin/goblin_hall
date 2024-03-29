use bevy::prelude::Query;

use crate::components::{CharacterSpawns, MapSpawns, StructureSpawns};

pub fn clear(
    mut character_spawns_query: Query<&mut CharacterSpawns>,
    mut map_spawns_query: Query<&mut MapSpawns>,
    mut structure_spawns_query: Query<&mut StructureSpawns>,
) {
    if let Ok(mut character_spawns) = character_spawns_query.get_single_mut() {
        character_spawns.clear();
    }

    if let Ok(mut map_spawns) = map_spawns_query.get_single_mut() {
        map_spawns.clear();
    }

    if let Ok(mut structure_spawns) = structure_spawns_query.get_single_mut() {
        structure_spawns.clear();
    }
}
