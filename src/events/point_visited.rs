use bevy::prelude::{Event, Vec2};

#[derive(Clone, Debug, Event)]
pub struct PointVisited {
    pub point: Vec2,
}
