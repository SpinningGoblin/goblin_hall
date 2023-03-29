use bevy::{
    prelude::{default, info, AssetServer, Commands, Query, Res, Transform, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::{
    components::{structures::StorageArea, Map, StructureSpawns},
    resources::{
        config::{grid::world_coordinate_from_grid, GameConfiguration},
        sprites::Atlas,
    },
};

pub fn structures(
    mut commands: Commands,
    structure_spawns_query: Query<&StructureSpawns>,
    atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfiguration>,
    map_query: Query<&Map>,
) {
    let (Ok(structure_spawns), Ok(map)) = (structure_spawns_query.get_single(), map_query.get_single()) else {
        return;
    };

    for spawnable in structure_spawns.spawnables.iter() {
        let coordinate = world_coordinate_from_grid(
            &spawnable.spawn_coordinate.coordinate,
            map.grid_size,
            map.tile_size,
        );
        match spawnable.spawn_type {
            crate::components::StructureSpawnType::StorageArea => {
                if let Some(zone_config) = game_config.zone_config("storage_area") {
                    info!("{}", &zone_config.overlay.path);
                    let structure_handle = asset_server.get_handle(&zone_config.overlay.path);
                    let target_index = atlas
                        .texture_atlas
                        .get_texture_index(&structure_handle)
                        .unwrap();
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    coordinate.x,
                                    coordinate.y,
                                    spawnable.spawn_coordinate.z_level,
                                ),
                                scale: Vec3::splat(game_config.tile_scale()),
                                ..default()
                            },
                            visibility: spawnable.visibility,
                            sprite: TextureAtlasSprite::new(target_index),
                            texture_atlas: atlas.atlas_handle.clone(),
                            ..default()
                        })
                        .insert(StorageArea {});
                }
            }
        }
    }
}
