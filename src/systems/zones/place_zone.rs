use bevy::{
    prelude::{
        default, AssetServer, Commands, Input, MouseButton, Query, Res, Transform, Vec3,
        Visibility, With, Without,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::{
    components::{target::MouseTarget, zones::Zone},
    resources::{config::GameConfiguration, sprites::Atlas},
};

pub fn place_zone(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    target_query: Query<(&Transform, &Visibility), With<MouseTarget>>,
    mut zone_query: Query<(&mut Transform, &Zone), Without<MouseTarget>>,
    game_config: Res<GameConfiguration>,
    atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
) {
    if target_query.is_empty() || !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let (target_transform, visibility) = target_query.single();

    if !visibility.is_visible {
        return;
    }

    if zone_query.is_empty() {
        let target_handle = asset_server.get_handle(&game_config.zone().path);
        let target_index = atlas
            .texture_atlas
            .get_texture_index(&target_handle)
            .unwrap();
        commands
            .spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: target_transform.translation,
                    scale: Vec3::splat(game_config.tile_scale()),
                    ..default()
                },
                sprite: TextureAtlasSprite::new(target_index),
                texture_atlas: atlas.atlas_handle.clone(),
                ..default()
            })
            .insert(Zone);
    } else {
        let (mut transform, _) = zone_query.single_mut();
        transform.translation.x = target_transform.translation.x;
        transform.translation.y = target_transform.translation.y;
    }
}
