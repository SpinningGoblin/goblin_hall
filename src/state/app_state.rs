use bevy::prelude::States;

#[derive(Clone, Debug, PartialEq, Eq, Hash, States)]
pub enum AppState {
    Startup,
    InGame,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Startup
    }
}
