use bevy::prelude::SystemSet;

// System sets to enforce a run order of our systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sets {
    Input,
    InputResponse,
    CharacterJobs,
    CharacterCleanup,
    Tick,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StartupSets {
    TextureAtlas,
}
