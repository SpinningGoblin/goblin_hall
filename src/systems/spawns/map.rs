use bevy::{
    prelude::{default, info, AssetServer, Commands, Query, Res, Transform, Vec3, Visibility},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};
use tdlg::map::layers::{FloorType, LayerType, StructureType};

use crate::{
    components::{
        resources::Resource,
        structures::{GridBody, Mineable, Structure},
        zones::ZoneType,
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

    for spawnable in map_spawns.zone_spawnables.iter() {
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

        let key = match spawnable.zone_type {
            ZoneType::Exploration => "exploration",
            ZoneType::SetupStorageArea => "setup_storage",
        };

        if let Some(zone_config) = game_config.zone_config(key) {
            let handle = asset_server.get_handle(&zone_config.overlay.path);
            let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
            commands
                .spawn(SpriteSheetBundle {
                    transform,
                    sprite: TextureAtlasSprite::new(texture_index),
                    texture_atlas: atlas.atlas_handle.clone(),
                    visibility: Visibility::Inherited,
                    ..default()
                })
                .insert(GridBody {
                    center_coordinate: spawnable.spawn_coordinate.coordinate,
                });
        }
    }

    for spawnable in map_spawns.tdlg_spawnables.iter() {
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
        let possible_spawn_commands = match spawnable.layer_type {
            LayerType::Floor(FloorType::Outdoor) => game_config
                .random_floor_sprite("cave_floor")
                .map(|floor_sprite| {
                    let handle = asset_server.get_handle(&floor_sprite.path);
                    let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                    commands.spawn(SpriteSheetBundle {
                        transform,
                        sprite: TextureAtlasSprite::new(texture_index),
                        texture_atlas: atlas.atlas_handle.clone(),
                        visibility: spawnable.visibility,
                        ..default()
                    })
                }),
            LayerType::Floor(FloorType::Indoor) => game_config
                .random_floor_sprite("dirt_floor")
                .map(|floor_sprite| {
                    let handle = asset_server.get_handle(&floor_sprite.path);
                    let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                    commands.spawn(SpriteSheetBundle {
                        transform,
                        sprite: TextureAtlasSprite::new(texture_index),
                        texture_atlas: atlas.atlas_handle.clone(),
                        visibility: spawnable.visibility,
                        ..default()
                    })
                }),
            LayerType::Structure(StructureType::Wall) => game_config
                .structure_config_by_key("room_wall")
                .and_then(|structure_config| structure_config.max_health_sprite())
                .map(|wall_sprite| {
                    let handle = asset_server.get_handle(&wall_sprite.path);
                    let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                    let mut spawn_commands = commands.spawn(SpriteSheetBundle {
                        transform,
                        sprite: TextureAtlasSprite::new(texture_index),
                        texture_atlas: atlas.atlas_handle.clone(),
                        visibility: spawnable.visibility,
                        ..default()
                    });

                    spawn_commands
                        .insert(Structure {
                            layer_type: spawnable.layer_type,
                        })
                        .insert(Mineable {
                            layer_type: spawnable.layer_type,
                            provides: Resource::Stone(2),
                        });
                    spawn_commands
                }),
            LayerType::Structure(StructureType::Door) => game_config
                .random_floor_sprite("sand_floor")
                .map(|floor_sprite| {
                    let handle = asset_server.get_handle(&floor_sprite.path);
                    let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                    let mut spawn_commands = commands.spawn(SpriteSheetBundle {
                        transform,
                        sprite: TextureAtlasSprite::new(texture_index),
                        texture_atlas: atlas.atlas_handle.clone(),
                        visibility: spawnable.visibility,
                        ..default()
                    });

                    spawn_commands.insert(Structure {
                        layer_type: LayerType::Structure(StructureType::Wall),
                    });
                    spawn_commands
                }),
            LayerType::Structure(StructureType::Boulder) => game_config
                .structure_config_by_key("outer_wall")
                .and_then(|structure_config| structure_config.max_health_sprite())
                .map(|wall_sprite| {
                    let handle = asset_server.get_handle(&wall_sprite.path);
                    let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                    let mut spawn_commands = commands.spawn(SpriteSheetBundle {
                        transform,
                        sprite: TextureAtlasSprite::new(texture_index),
                        texture_atlas: atlas.atlas_handle.clone(),
                        visibility: spawnable.visibility,
                        ..default()
                    });

                    spawn_commands.insert(Structure {
                        layer_type: LayerType::Structure(StructureType::Boulder),
                    });
                    spawn_commands
                }),
            LayerType::Structure(StructureType::Rubble) => game_config
                .structure_config_by_key("rubble")
                .and_then(|structure_config| structure_config.max_health_sprite())
                .map(|rubble_sprite| {
                    let handle = asset_server.get_handle(&rubble_sprite.path);
                    let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                    let mut spawn_commands = commands.spawn(SpriteSheetBundle {
                        transform,
                        sprite: TextureAtlasSprite::new(texture_index),
                        texture_atlas: atlas.atlas_handle.clone(),
                        visibility: spawnable.visibility,
                        ..default()
                    });

                    spawn_commands.insert(Structure {
                        layer_type: LayerType::Structure(StructureType::Rubble),
                    });
                    spawn_commands
                }),
            LayerType::Structure(StructureType::Table) => game_config
                .structure_config_by_key("table")
                .and_then(|structure_config| structure_config.max_health_sprite())
                .map(|table_sprite| {
                    let handle = asset_server.get_handle(&table_sprite.path);
                    let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                    commands.spawn(SpriteSheetBundle {
                        transform,
                        sprite: TextureAtlasSprite::new(texture_index),
                        texture_atlas: atlas.atlas_handle.clone(),
                        visibility: spawnable.visibility,
                        ..default()
                    })
                }),
            LayerType::Structure(structure_type) => {
                info!("Spawning structure {:?}", structure_type);
                None
            }
            LayerType::Exit => {
                info!("Spawning exit");
                None
            }
            LayerType::Item(_)
            | LayerType::Note
            | LayerType::Path
            | LayerType::Entrance
            | LayerType::Empty => None,
        };

        if let Some(mut spawn_commands) = possible_spawn_commands {
            spawn_commands.insert(GridBody {
                center_coordinate: spawnable.spawn_coordinate.coordinate,
            });

            if let Some(resource) = &spawnable.resource {
                spawn_commands.insert(resource.clone());
            }
        }
    }
}
