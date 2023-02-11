use bevy::prelude::SystemLabel;

// System labels to enforce a run order of our systems
#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Label {
    CameraMovement,
    CameraMovementInput,
}

#[derive(SystemLabel, Debug)]
pub enum StartupLabels {
    TextureAtlas,
}
