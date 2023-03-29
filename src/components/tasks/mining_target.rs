use bevy::prelude::Entity;
use tdlg::map::{cells::Coordinate, layers::LayerType};

#[derive(Clone)]
pub struct MiningTarget {
    pub entity: Option<Entity>,
    pub layer_type: Option<LayerType>,
    pub coordinate: Coordinate,
}
