use bevy::prelude::{HandleUntyped, Resource};

#[derive(Default, Resource)]
pub struct Handles {
    pub all: Vec<HandleUntyped>,
}
