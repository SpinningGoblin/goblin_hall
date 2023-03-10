use bevy::prelude::States;

#[derive(Clone, Debug, PartialEq, Eq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Startup,
    InGame,
}
