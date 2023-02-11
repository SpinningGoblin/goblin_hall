use bevy::{
    prelude::{warn, AssetServer, Assets, Commands, Image, Res, ResMut},
    sprite::{TextureAtlas, TextureAtlasBuilder},
};

use crate::resources::sprites::{Atlas, Handles};

pub fn finalize_texture_atlas(
    mut commands: Commands,
    handles: Res<Handles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &handles.all {
        let handle = handle.typed_weak();
        let Some(texture) = textures.get(&handle) else {
            warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
            continue;
        };

        texture_atlas_builder.add_texture(handle, texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas.clone());

    commands.insert_resource(Atlas {
        atlas_handle,
        texture_atlas,
    });
}
