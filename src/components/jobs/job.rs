use bevy::prelude::Component;

#[derive(Component)]
pub enum Job {
    Miner,
    Explorer,
}
