use bevy::{
    prelude::{default, AssetServer, Commands, Res, ResMut, Transform, Vec3, Visibility},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::{
    components::target::MouseTarget,
    resources::{config::GameConfiguration, sprites::Atlas},
};

pub fn spawn(
    mut commands: Commands,
    atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
    game_config: ResMut<GameConfiguration>,
) {
    let target_handle = asset_server.get_handle(&game_config.mouse_target().path);
    let target_index = atlas
        .texture_atlas
        .get_texture_index(&target_handle)
        .unwrap();
    commands
        .spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::splat(10.0),
                scale: Vec3::splat(game_config.tile_scale()),
                ..default()
            },
            sprite: TextureAtlasSprite::new(target_index),
            texture_atlas: atlas.atlas_handle.clone(),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(MouseTarget);
}
