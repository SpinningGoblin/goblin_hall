use bevy::{
    prelude::{AssetServer, Input, KeyCode, Query, Res, With},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{target::MouseTarget, zones::ZoneType},
    resources::{config::GameConfiguration, sprites::Atlas},
};

pub fn swap_targets(
    mut query: Query<(&mut ZoneType, &mut TextureAtlasSprite), With<MouseTarget>>,
    keyboard_input: Res<Input<KeyCode>>,
    game_config: Res<GameConfiguration>,
    atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
) {
    let Ok((mut zone_type, mut sprite)) = query.get_single_mut() else {
        return;
    };

    let (new_zone_type, key) = if keyboard_input.just_pressed(KeyCode::E) {
        (ZoneType::Exploration, "exploration")
    } else if keyboard_input.just_pressed(KeyCode::S) {
        (ZoneType::SetupStorageArea, "setup_storage")
    } else {
        return;
    };

    *zone_type = new_zone_type;

    let zone_config = game_config.zone_config(key).unwrap();
    let target_handle = asset_server.get_handle(&zone_config.target.path);
    let target_index = atlas
        .texture_atlas
        .get_texture_index(&target_handle)
        .unwrap();
    sprite.index = target_index;
}
