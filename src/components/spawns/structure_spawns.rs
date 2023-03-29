use bevy::prelude::{Component, Visibility};

use super::SpawnCoordinate;

#[derive(Component, Debug, Clone)]
pub struct StructureSpawns {
    pub spawnables: Vec<StructureSpawnable>,
}

impl StructureSpawns {
    pub fn clear(&mut self) {
        self.spawnables.clear();
    }
}

#[derive(Clone, Debug)]
pub struct StructureSpawnable {
    pub spawn_type: StructureSpawnType,
    pub spawn_coordinate: SpawnCoordinate,
    pub visibility: Visibility,
}

#[derive(Clone, Copy, Debug)]
pub enum StructureSpawnType {
    StorageArea,
}
