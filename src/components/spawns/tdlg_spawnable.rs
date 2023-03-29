use bevy::prelude::Visibility;
use tdlg::map::layers::LayerType;

use super::SpawnCoordinate;

pub struct TdlgSpawnable {
    pub layer_type: LayerType,
    pub spawn_coordinate: SpawnCoordinate,
    pub visibility: Visibility,
}
