use bevy::prelude::{Component, Vec2};
use tdlg::map::layers::LayerType;

#[derive(Component, Debug)]
pub struct MoveTarget {
    pub point: Vec2,
    pub layer_type: LayerType,
    pub path: Vec<Vec2>,
}
