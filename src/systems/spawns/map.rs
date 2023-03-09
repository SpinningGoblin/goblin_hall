use bevy::{
    prelude::{default, info, AssetServer, Commands, Query, Res, Transform, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};
use tdlg::map::layers::{FloorType, LayerType, StructureType};

use crate::{
    components::{
        structures::{GridBody, Mineable, Structure},
        Map, MapSpawns,
    },
    resources::{
        config::{grid::world_coordinate_from_grid, GameConfiguration},
        sprites::Atlas,
    },
};

pub fn map(
    mut commands: Commands,
    map_spawns_query: Query<&MapSpawns>,
    atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfiguration>,
    map_query: Query<&Map>,
) {
    let (Ok(map_spawns), Ok(map)) = (map_spawns_query.get_single(), map_query.get_single()) else {
        return;
    };

    for spawnable in map_spawns.spawnables.iter() {
        let coordinate = world_coordinate_from_grid(
            &spawnable.spawn_coordinate.coordinate,
            map.grid_size,
            map.tile_size,
        );
        let transform = Transform {
            translation: Vec3::new(
                coordinate.x,
                coordinate.y,
                spawnable.spawn_coordinate.z_level,
            ),
            scale: Vec3::splat(game_config.tile_scale()),
            ..default()
        };
        match spawnable.layer_type {
            LayerType::Floor(FloorType::Outdoor) => {
                if let Some(floor_sprite) = game_config.random_floor_sprite("cave_floor") {
                    let handle = asset_server.get_handle(&floor_sprite.path);
                    let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                    commands
                        .spawn(SpriteSheetBundle {
                            transform,
                            sprite: TextureAtlasSprite::new(texture_index),
                            texture_atlas: atlas.atlas_handle.clone(),
                            visibility: spawnable.visibility,
                            ..default()
                        })
                        .insert(GridBody {
                            center_coordinate: spawnable.spawn_coordinate.coordinate,
                        });
                }
            }
            LayerType::Floor(FloorType::Indoor) => {
                if let Some(floor_sprite) = game_config.random_floor_sprite("dirt_floor") {
                    let handle = asset_server.get_handle(&floor_sprite.path);
                    let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                    commands
                        .spawn(SpriteSheetBundle {
                            transform,
                            sprite: TextureAtlasSprite::new(texture_index),
                            texture_atlas: atlas.atlas_handle.clone(),
                            visibility: spawnable.visibility,
                            ..default()
                        })
                        .insert(GridBody {
                            center_coordinate: spawnable.spawn_coordinate.coordinate,
                        });
                }
            }
            LayerType::Structure(StructureType::Wall) => {
                if let Some(structure_config) = game_config.structure_config_by_key("room_wall") {
                    if let Some(wall_sprite) = structure_config.max_health_sprite() {
                        let handle = asset_server.get_handle(&wall_sprite.path);
                        let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                        commands
                            .spawn(SpriteSheetBundle {
                                transform,
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas.atlas_handle.clone(),
                                visibility: spawnable.visibility,
                                ..default()
                            })
                            .insert(Structure {
                                layer_type: spawnable.layer_type,
                            })
                            .insert(Mineable {
                                layer_type: spawnable.layer_type,
                            })
                            .insert(GridBody {
                                center_coordinate: spawnable.spawn_coordinate.coordinate,
                            });
                    }
                }
            }
            LayerType::Structure(StructureType::Door) => {
                if let Some(floor_sprite) = game_config.random_floor_sprite("sand_floor") {
                    let handle = asset_server.get_handle(&floor_sprite.path);
                    let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                    commands
                        .spawn(SpriteSheetBundle {
                            transform,
                            sprite: TextureAtlasSprite::new(texture_index),
                            texture_atlas: atlas.atlas_handle.clone(),
                            visibility: spawnable.visibility,
                            ..default()
                        })
                        .insert(Structure {
                            layer_type: LayerType::Structure(StructureType::Wall),
                        })
                        .insert(GridBody {
                            center_coordinate: spawnable.spawn_coordinate.coordinate,
                        });
                }
            }
            LayerType::Structure(StructureType::Boulder) => {
                if let Some(structure_config) = game_config.structure_config_by_key("outer_wall") {
                    if let Some(wall_sprite) = structure_config.max_health_sprite() {
                        let handle = asset_server.get_handle(&wall_sprite.path);
                        let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                        commands
                            .spawn(SpriteSheetBundle {
                                transform,
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas.atlas_handle.clone(),
                                visibility: spawnable.visibility,
                                ..default()
                            })
                            .insert(Structure {
                                layer_type: LayerType::Structure(StructureType::Boulder),
                            })
                            .insert(GridBody {
                                center_coordinate: spawnable.spawn_coordinate.coordinate,
                            });
                    }
                }
            }
            LayerType::Structure(StructureType::Rubble) => {
                if let Some(structure_config) = game_config.structure_config_by_key("rubble") {
                    if let Some(wall_sprite) = structure_config.max_health_sprite() {
                        let handle = asset_server.get_handle(&wall_sprite.path);
                        let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                        commands
                            .spawn(SpriteSheetBundle {
                                transform,
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas.atlas_handle.clone(),
                                visibility: spawnable.visibility,
                                ..default()
                            })
                            .insert(Structure {
                                layer_type: LayerType::Structure(StructureType::Rubble),
                            })
                            .insert(GridBody {
                                center_coordinate: spawnable.spawn_coordinate.coordinate,
                            });
                    }
                }
            }
            LayerType::Structure(StructureType::Table) => {
                if let Some(structure_config) = game_config.structure_config_by_key("table") {
                    if let Some(wall_sprite) = structure_config.max_health_sprite() {
                        let handle = asset_server.get_handle(&wall_sprite.path);
                        let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                        commands
                            .spawn(SpriteSheetBundle {
                                transform,
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas.atlas_handle.clone(),
                                visibility: spawnable.visibility,
                                ..default()
                            })
                            .insert(GridBody {
                                center_coordinate: spawnable.spawn_coordinate.coordinate,
                            });
                    }
                }
            }
            LayerType::Structure(structure_type) => {
                info!("Spawning structure {:?}", structure_type);
            }
            LayerType::Exit => {
                info!("Spawning exit");
            }
            LayerType::Item(_)
            | LayerType::Note
            | LayerType::Path
            | LayerType::Entrance
            | LayerType::Empty => {}
        }
    }
}
