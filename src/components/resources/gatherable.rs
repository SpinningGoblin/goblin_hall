use bevy::prelude::Component;

use super::Resource;

#[derive(Component, Clone, Debug)]
pub struct Gatherable {
    pub resource: Resource,
    pub targeted: bool,
}
