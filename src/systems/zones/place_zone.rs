use bevy::{
    prelude::{
        default, AssetServer, Commands, Input, MouseButton, Query, Res, Transform, Vec3,
        Visibility, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::{
    components::{structures::GridBody, target::MouseTarget, zones::Zone, Map},
    resources::{
        config::{grid::grid_coordinate_from_world, GameConfiguration},
        sprites::Atlas,
    },
};

pub fn place_zone(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    target_query: Query<(&Transform, &Visibility), With<MouseTarget>>,
    map_query: Query<&Map>,
    game_config: Res<GameConfiguration>,
    atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
) {
    let queries = (target_query.get_single(), map_query.get_single());
    let (Ok((target_transform, visibility)), Ok(map)) = queries else {
        return;
    };

    if !mouse_input.just_pressed(MouseButton::Left) || visibility == Visibility::Hidden {
        return;
    }

    let target_handle = asset_server.get_handle(&game_config.zone().path);
    let target_index = atlas
        .texture_atlas
        .get_texture_index(&target_handle)
        .unwrap();
    let point = target_transform.translation.truncate();
    let coordinate = grid_coordinate_from_world(&point, map.grid_size, map.tile_size);
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
        .insert(Zone)
        .insert(GridBody {
            center_coordinate: coordinate,
        });
}
