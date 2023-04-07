use bevy::prelude::Component;

use crate::components::movement::Direction;

#[derive(Component, Clone, Debug, Default)]
pub struct PreviousExplorations {
    pub direction: Option<Direction>,
}
