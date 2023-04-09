use bevy::prelude::Visibility;
use tdlg::map::layers::LayerType;

use crate::components::resources::Resource;

use super::SpawnCoordinate;

pub struct TdlgSpawnable {
    pub layer_type: LayerType,
    pub resource: Option<Resource>,
    pub spawn_coordinate: SpawnCoordinate,
    pub visibility: Visibility,
}
