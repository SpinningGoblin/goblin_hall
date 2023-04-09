use bevy::prelude::Component;

use crate::components::{
    resources::{Resource, ResourceBundle},
    structures::StorageArea,
};

#[derive(Component, Clone, Debug, Copy, Default)]
pub struct ResourceInventory {
    pub stone_count: u16,
}

impl ResourceInventory {
    pub fn add_resource(&mut self, resource: &Resource) {
        match resource {
            Resource::Stone(count) => {
                self.stone_count = match self.stone_count.checked_add(*count as u16) {
                    Some(it) => it,
                    None => u16::MAX,
                };
            }
        }
    }

    pub fn total(&self) -> u32 {
        self.stone_count as u32
    }

    pub fn empty(&mut self) {
        self.stone_count = 0;
    }

    pub fn empty_into(&mut self, storage_area: &mut StorageArea) {
        storage_area.add_resource_bundle(ResourceBundle::Stone(self.stone_count));

        self.empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::components::resources::Resource;

    use super::ResourceInventory;

    #[test]
    fn add_at_max() {
        let mut resource_inventory = ResourceInventory {
            stone_count: u16::MAX,
        };

        resource_inventory.add_resource(&Resource::Stone(u8::MAX));

        assert_eq!(resource_inventory.stone_count, u16::MAX);
    }
}
