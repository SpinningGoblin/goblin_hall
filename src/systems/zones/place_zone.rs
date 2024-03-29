use bevy::{
    prelude::{
        default, AssetServer, Commands, Input, MouseButton, Query, Res, Transform, Vec3,
        Visibility, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::{
    components::{
        structures::GridBody,
        target::MouseTarget,
        zones::{ExplorationZone, SetupStorageAreaZone, ZoneType},
        Map,
    },
    resources::{
        config::{grid::grid_coordinate_from_world, GameConfiguration},
        sprites::Atlas,
    },
};

pub fn place_zone(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    target_query: Query<(&Transform, &Visibility, &ZoneType), With<MouseTarget>>,
    map_query: Query<&Map>,
    game_config: Res<GameConfiguration>,
    atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
) {
    let queries = (target_query.get_single(), map_query.get_single());
    let (Ok((target_transform, visibility, zone_type)), Ok(map)) = queries else {
        return;
    };

    if !mouse_input.just_pressed(MouseButton::Left) || visibility == Visibility::Hidden {
        return;
    }

    let point = target_transform.translation.truncate();
    let coordinate = grid_coordinate_from_world(&point, map.grid_size, map.tile_size);
    let key = match zone_type {
        ZoneType::Exploration => "exploration",
        ZoneType::SetupStorageArea => "setup_storage",
    };
    let zone_config = game_config.zone_config(key).unwrap();
    let target_handle = asset_server.get_handle(&zone_config.overlay.path);
    let target_index = atlas
        .texture_atlas
        .get_texture_index(&target_handle)
        .unwrap();

    let mut spawn_command = commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: target_transform.translation,
            scale: Vec3::splat(game_config.tile_scale()),
            ..default()
        },
        sprite: TextureAtlasSprite::new(target_index),
        texture_atlas: atlas.atlas_handle.clone(),
        ..default()
    });
    spawn_command.insert(GridBody {
        center_coordinate: coordinate,
    });

    match zone_type {
        ZoneType::Exploration => spawn_command.insert(ExplorationZone::default()),
        ZoneType::SetupStorageArea => spawn_command.insert(SetupStorageAreaZone::default()),
    };
}
