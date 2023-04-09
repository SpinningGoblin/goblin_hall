use bevy::prelude::Component;

use crate::components::resources::ResourceBundle;

#[derive(Clone, Component, Debug, Default)]
pub struct StorageArea {
    pub stone_count: u128,
}

impl StorageArea {
    pub fn add_resource_bundle(&mut self, resource: ResourceBundle) {
        match resource {
            ResourceBundle::Stone(count) => {
                self.stone_count = match self.stone_count.checked_add(count as u128) {
                    Some(it) => it,
                    None => u128::MAX,
                }
            }
        }
    }
}
