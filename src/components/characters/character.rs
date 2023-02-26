use std::num::NonZeroI32;

use bevy::prelude::Component;
use tdlg::map::cells::Coordinate;

use crate::components::GridBox;

#[derive(Component)]
pub struct Character {
    pub visibility: NonZeroI32,
}

impl Character {
    pub fn visibility_box(&self, center: Coordinate) -> GridBox {
        GridBox {
            center,
            size: self.visibility.get(),
        }
    }
}
