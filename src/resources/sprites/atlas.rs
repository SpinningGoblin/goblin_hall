use bevy::{
    prelude::{Handle, Resource},
    sprite::TextureAtlas,
};

#[derive(Resource)]
pub struct Atlas {
    pub atlas_handle: Handle<TextureAtlas>,
    pub texture_atlas: TextureAtlas,
}
