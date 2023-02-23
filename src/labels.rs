use bevy::prelude::SystemLabel;

// System labels to enforce a run order of our systems
#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Labels {
    Input,
    InputResponse,
    CharacterJobs,
    Tick,
}

#[derive(SystemLabel, Debug)]
pub enum StartupLabels {
    TextureAtlas,
}
