use bevy::prelude::Component;

#[derive(Component, Clone)]
pub enum Resource {
    Stone(u8),
}

#[derive(Component, Clone)]
pub enum ResourceBundle {
    Stone(u16),
}
