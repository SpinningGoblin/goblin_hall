use bevy::prelude::Component;
use tdlg::map::layers::LayerType;

#[derive(Component)]
pub struct Structure {
    pub layer_type: LayerType,
}
