use bevy::{
    prelude::{default, info, AssetServer, Commands, Res, ResMut, Transform, Vec3, Visibility},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};
use tdlg::map::layers::{FloorType, LayerType, StructureType};

use crate::{
    components::{
        characters::Character,
        jobs::ExplorationHistory,
        movement::CameraMoveTimer,
        structures::{Body, Mineable, Structure},
        Map, World,
    },
    resources::{
        config::{grid::world_coordinate_from_grid, GameConfiguration},
        sprites::Atlas,
    },
};

pub fn spawn_starting(
    mut commands: Commands,
    atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
    mut game_config: ResMut<GameConfiguration>,
) {
    commands.spawn(World::default());
    commands.spawn(CameraMoveTimer {
        timer: game_config.movement_timer(),
    });

    commands.spawn(ExplorationHistory::default());

    let top_down_map = game_config.generate_top_down_map();
    for cell in top_down_map.grid().cells() {
        let coordinate = world_coordinate_from_grid(
            cell.coordinate(),
            game_config.grid_size().get(),
            game_config.tile_size(),
        );
        for (index, layer) in cell.layers().iter().enumerate() {
            let position = Vec3::new(coordinate.x, coordinate.y, index as f32);
            match *layer {
                LayerType::Floor(FloorType::Outdoor) => {
                    if let Some(floor_sprite) = game_config.random_floor_sprite("cave_floor") {
                        let handle = asset_server.get_handle(&floor_sprite.path);
                        let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                        commands
                            .spawn(SpriteSheetBundle {
                                transform: Transform {
                                    translation: position,
                                    scale: Vec3::splat(game_config.tile_scale()),
                                    ..default()
                                },
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas.atlas_handle.clone(),
                                visibility: Visibility { is_visible: false },
                                ..default()
                            })
                            .insert(Body {
                                tile_size: game_config.tile_size(),
                                cell_center: position.truncate(),
                                underground: false,
                                center_coordinate: *cell.coordinate(),
                            });
                    }
                }
                LayerType::Structure(StructureType::Wall) => {
                    if let Some(structure_config) = game_config.structure_config_by_key("room_wall")
                    {
                        if let Some(wall_sprite) = structure_config.max_health_sprite() {
                            let handle = asset_server.get_handle(&wall_sprite.path);
                            let texture_index =
                                atlas.texture_atlas.get_texture_index(&handle).unwrap();
                            commands
                                .spawn(SpriteSheetBundle {
                                    transform: Transform {
                                        translation: position,
                                        scale: Vec3::splat(game_config.tile_scale()),
                                        ..default()
                                    },
                                    sprite: TextureAtlasSprite::new(texture_index),
                                    texture_atlas: atlas.atlas_handle.clone(),
                                    visibility: Visibility { is_visible: false },
                                    ..default()
                                })
                                .insert(Structure {
                                    layer_type: LayerType::Structure(StructureType::Wall),
                                })
                                .insert(Mineable {
                                    layer_type: LayerType::Structure(StructureType::Wall),
                                })
                                .insert(Body {
                                    tile_size: game_config.tile_size(),
                                    cell_center: position.truncate(),
                                    underground: false,
                                    center_coordinate: *cell.coordinate(),
                                });
                        }
                    }
                }
                LayerType::Floor(FloorType::Indoor) => {
                    if let Some(floor_sprite) = game_config.random_floor_sprite("dirt_floor") {
                        let handle = asset_server.get_handle(&floor_sprite.path);
                        let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                        commands
                            .spawn(SpriteSheetBundle {
                                transform: Transform {
                                    translation: position,
                                    scale: Vec3::splat(game_config.tile_scale()),
                                    ..default()
                                },
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas.atlas_handle.clone(),
                                visibility: Visibility { is_visible: false },
                                ..default()
                            })
                            .insert(Body {
                                tile_size: game_config.tile_size(),
                                cell_center: position.truncate(),
                                underground: false,
                                center_coordinate: *cell.coordinate(),
                            });
                    }
                }
                LayerType::Structure(StructureType::Door) => {
                    if let Some(floor_sprite) = game_config.random_floor_sprite("sand_floor") {
                        let handle = asset_server.get_handle(&floor_sprite.path);
                        let texture_index = atlas.texture_atlas.get_texture_index(&handle).unwrap();
                        commands
                            .spawn(SpriteSheetBundle {
                                transform: Transform {
                                    translation: position,
                                    scale: Vec3::splat(game_config.tile_scale()),
                                    ..default()
                                },
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas.atlas_handle.clone(),
                                visibility: Visibility { is_visible: false },
                                ..default()
                            })
                            .insert(Structure {
                                layer_type: LayerType::Structure(StructureType::Wall),
                            })
                            .insert(Body {
                                tile_size: game_config.tile_size(),
                                cell_center: position.truncate(),
                                underground: false,
                                center_coordinate: *cell.coordinate(),
                            });
                    }
                }
                LayerType::Structure(StructureType::Boulder) => {
                    if let Some(structure_config) =
                        game_config.structure_config_by_key("outer_wall")
                    {
                        if let Some(wall_sprite) = structure_config.max_health_sprite() {
                            let handle = asset_server.get_handle(&wall_sprite.path);
                            let texture_index =
                                atlas.texture_atlas.get_texture_index(&handle).unwrap();
                            commands
                                .spawn(SpriteSheetBundle {
                                    transform: Transform {
                                        translation: position,
                                        scale: Vec3::splat(game_config.tile_scale()),
                                        ..default()
                                    },
                                    sprite: TextureAtlasSprite::new(texture_index),
                                    texture_atlas: atlas.atlas_handle.clone(),
                                    visibility: Visibility { is_visible: false },
                                    ..default()
                                })
                                .insert(Structure {
                                    layer_type: LayerType::Structure(StructureType::Boulder),
                                })
                                .insert(Body {
                                    tile_size: game_config.tile_size(),
                                    cell_center: position.truncate(),
                                    underground: false,
                                    center_coordinate: *cell.coordinate(),
                                });
                        }
                    }
                }
                LayerType::Structure(StructureType::Rubble) => {
                    if let Some(structure_config) = game_config.structure_config_by_key("rubble") {
                        if let Some(wall_sprite) = structure_config.max_health_sprite() {
                            let handle = asset_server.get_handle(&wall_sprite.path);
                            let texture_index =
                                atlas.texture_atlas.get_texture_index(&handle).unwrap();
                            commands
                                .spawn(SpriteSheetBundle {
                                    transform: Transform {
                                        translation: position,
                                        scale: Vec3::splat(game_config.tile_scale()),
                                        ..default()
                                    },
                                    sprite: TextureAtlasSprite::new(texture_index),
                                    texture_atlas: atlas.atlas_handle.clone(),
                                    visibility: Visibility { is_visible: false },
                                    ..default()
                                })
                                .insert(Structure {
                                    layer_type: LayerType::Structure(StructureType::Rubble),
                                })
                                .insert(Body {
                                    tile_size: game_config.tile_size(),
                                    cell_center: position.truncate(),
                                    underground: false,
                                    center_coordinate: *cell.coordinate(),
                                });
                        }
                    }
                }
                LayerType::Structure(StructureType::Table) => {
                    if let Some(structure_config) = game_config.structure_config_by_key("table") {
                        if let Some(wall_sprite) = structure_config.max_health_sprite() {
                            let handle = asset_server.get_handle(&wall_sprite.path);
                            let texture_index =
                                atlas.texture_atlas.get_texture_index(&handle).unwrap();
                            commands
                                .spawn(SpriteSheetBundle {
                                    transform: Transform {
                                        translation: position,
                                        scale: Vec3::splat(game_config.tile_scale()),
                                        ..default()
                                    },
                                    sprite: TextureAtlasSprite::new(texture_index),
                                    texture_atlas: atlas.atlas_handle.clone(),
                                    visibility: Visibility { is_visible: false },
                                    ..default()
                                })
                                .insert(Body {
                                    tile_size: game_config.tile_size(),
                                    cell_center: position.truncate(),
                                    underground: false,
                                    center_coordinate: *cell.coordinate(),
                                });
                        }
                    }
                }
                LayerType::Note => {
                    info!("Note {} {:?}", index, &cell.coordinate());
                }
                LayerType::Item(rarity) => {
                    info!(
                        "common item {} {:?} {:?}",
                        index,
                        &cell.coordinate(),
                        rarity
                    );
                }
                LayerType::Structure(it) => {
                    info!("structure {:?} {:?}", &cell.coordinate(), it);
                }
                LayerType::Empty | LayerType::Entrance | LayerType::Exit | LayerType::Path => {}
            }
        }
    }

    if let Some(character_config) = game_config.character_config("little_goblin") {
        let goblin_handle = asset_server.get_handle(&character_config.sprite.path);
        let target_index = atlas
            .texture_atlas
            .get_texture_index(&goblin_handle)
            .unwrap();
        let coordinate = world_coordinate_from_grid(
            top_down_map.entry(),
            game_config.grid_size().get(),
            game_config.tile_size(),
        );
        info!("entry {:?}", top_down_map.entry());
        commands
            .spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(coordinate.x, coordinate.y, 10.0),
                    scale: Vec3::splat(game_config.tile_scale()),
                    ..default()
                },
                sprite: TextureAtlasSprite::new(target_index),
                texture_atlas: atlas.atlas_handle.clone(),
                ..default()
            })
            .insert(Character {
                visibility: character_config.visibility,
            });

        commands.spawn(Map {
            current: top_down_map,
            tile_size: game_config.tile_size(),
            grid_size: game_config.grid_size().get(),
        });
    }
}
