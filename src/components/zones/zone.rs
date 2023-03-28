use bevy::prelude::Component;

/// Zones can be placed by the player to mark a place on the map
/// for the characters to go and do something, either clear as an
/// exploration target or set up something.
#[derive(Component)]
pub struct Zone;
