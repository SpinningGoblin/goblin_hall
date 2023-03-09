use bevy::prelude::Query;

use crate::components::{CharacterSpawns, MapSpawns};

pub fn clear(
    mut character_spawns_query: Query<&mut CharacterSpawns>,
    mut map_spawns_query: Query<&mut MapSpawns>,
) {
    if let Ok(mut character_spawns) = character_spawns_query.get_single_mut() {
        character_spawns.spawnables.clear();
    }

    if let Ok(mut map_spawns) = map_spawns_query.get_single_mut() {
        map_spawns.spawnables.clear();
    }
}
