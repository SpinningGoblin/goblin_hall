use bevy::prelude::{Component, Visibility};
use tdlg::map::{cells::Coordinate, layers::LayerType};

use super::{characters::CreatureType, zones::ZoneType};

#[derive(Component)]
pub struct CharacterSpawns {
    pub spawnables: Vec<CharacterSpawnable>,
}

#[derive(Component)]
pub struct MapSpawns {
    pub map_spawnables: Vec<MapSpawnable>,
    pub zone_spawnables: Vec<ZoneSpawnable>,
}

impl MapSpawns {
    pub fn clear(&mut self) {
        self.map_spawnables.clear();
        self.zone_spawnables.clear();
    }
}

pub struct ZoneSpawnable {
    pub spawn_coordinate: SpawnCoordinate,
    pub zone_type: ZoneType,
}

pub struct SpawnCoordinate {
    pub coordinate: Coordinate,
    pub z_level: f32,
}

pub struct MapSpawnable {
    pub layer_type: LayerType,
    pub spawn_coordinate: SpawnCoordinate,
    pub visibility: Visibility,
}

pub struct CharacterSpawnable {
    pub spawn_type: CreatureType,
    pub coordinate: SpawnCoordinate,
}
