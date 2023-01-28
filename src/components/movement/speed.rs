use bevy::prelude::Vec2;

#[derive(Debug, Clone)]
pub struct Speed {
    pub current: Vec2,
}

impl Default for Speed {
    fn default() -> Self {
        Self {
            current: Vec2::splat(0.0),
        }
    }
}
