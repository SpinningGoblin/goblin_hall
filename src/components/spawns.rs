use bevy::prelude::{Component, Visibility};
use tdlg::map::{cells::Coordinate, layers::LayerType};

use super::characters::CreatureType;

#[derive(Component)]
pub struct CharacterSpawns {
    pub spawnables: Vec<CharacterSpawnable>,
}

#[derive(Component)]
pub struct MapSpawns {
    pub spawnables: Vec<MapSpawnable>,
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
