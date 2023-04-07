use bevy::prelude::Component;

use super::ZoneType;

/// Zones can be placed by the player to mark a place on the map
/// for the characters to go and do something, either clear as an
/// exploration target or set up something.
pub trait Zone {
    fn zone_type(&self) -> ZoneType;
}

#[derive(Component, Clone, Copy, Debug)]
pub struct ExplorationZone;

impl Zone for ExplorationZone {
    fn zone_type(&self) -> ZoneType {
        ZoneType::Exploration
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct SetupStorageAreaZone;

impl Zone for SetupStorageAreaZone {
    fn zone_type(&self) -> ZoneType {
        ZoneType::SetupStorageArea
    }
}
