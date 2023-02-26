use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct World {
    pub tick_just_finished: bool,
}
