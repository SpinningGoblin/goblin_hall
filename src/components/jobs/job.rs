use bevy::prelude::Component;

#[derive(Component, Clone)]
pub enum Job {
    Miner,
    Explorer,
}
