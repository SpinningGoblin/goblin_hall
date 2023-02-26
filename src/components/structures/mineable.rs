use bevy::prelude::Component;
use tdlg::map::layers::LayerType;

#[derive(Component)]
pub struct Mineable {
    pub layer_type: LayerType,
}
