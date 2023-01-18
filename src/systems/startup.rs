use bevy::{
    prelude::{
        default, warn, AssetServer, Assets, Camera2dBundle, Commands, Image, Res, ResMut,
        Transform, Vec3,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite},
};

use crate::resources::sprites::Handles;

pub fn startup(
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
    let goblin_handle = asset_server.get_handle("sprites/goblin.png");
    let goblin_index = texture_atlas.get_texture_index(&goblin_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    // set up a scene to display our texture atlas
    commands.spawn(Camera2dBundle::default());
    // draw a sprite from the atlas
    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(150.0, 0.0, 0.0),
            scale: Vec3::splat(4.0),
            ..default()
        },
        sprite: TextureAtlasSprite::new(goblin_index),
        texture_atlas: atlas_handle,
        ..default()
    });
}
