use bevy::prelude::Component;

#[derive(Component, Clone, Debug, Copy)]
pub enum ZoneType {
    Exploration,
    SetupStorageArea,
    StorageArea,
}
