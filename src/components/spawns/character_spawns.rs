use bevy::prelude::Component;

use crate::components::characters::CreatureType;

use super::SpawnCoordinate;

#[derive(Component)]
pub struct CharacterSpawns {
    pub spawnables: Vec<CharacterSpawnable>,
}

impl CharacterSpawns {
    pub fn clear(&mut self) {
        self.spawnables.clear();
    }
}

pub struct CharacterSpawnable {
    pub spawn_type: CreatureType,
    pub coordinate: SpawnCoordinate,
}
