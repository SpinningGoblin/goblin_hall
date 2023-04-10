use bevy::prelude::Component;
use tdlg::map::layers::LayerType;

use crate::components::resources::Resource;

#[derive(Component)]
pub struct Mineable {
    pub layer_type: LayerType,
    pub provides: Resource,
    pub targeted: bool,
}
