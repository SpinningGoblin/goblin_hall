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

#[derive(SystemLabel, Debug)]
pub enum MouseLabels {
    Movement,
}

#[derive(SystemLabel, Debug)]
pub enum ZoneLabels {
    PlaceZone,
}

#[derive(SystemLabel, Debug)]
pub enum WorldLabels {
    TickWorld,
    CheckActions,
}
