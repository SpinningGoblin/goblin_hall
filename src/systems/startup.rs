use bevy::{
    prelude::{
        default, warn, AssetServer, Assets, Commands, Image, Res, ResMut, Transform, Vec3,
        Visibility,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite},
};
use tdlg::map::cells::LayerType;

use crate::{
    components::target::MouseTarget,
    resources::{
        config::{grid::world_coordinate_from_grid, GameConfiguration},
        sprites::Handles,
    },
};

pub fn startup(
    mut commands: Commands,
    handles: Res<Handles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut game_config: ResMut<GameConfiguration>,
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
    let atlas_handle = texture_atlases.add(texture_atlas.clone());

    let target_handle = asset_server.get_handle(&game_config.mouse_target().path);
    let target_index = texture_atlas.get_texture_index(&target_handle).unwrap();
    commands
        .spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::splat(10.0),
                scale: Vec3::splat(game_config.tile_scale()),
                ..default()
            },
            sprite: TextureAtlasSprite::new(target_index),
            texture_atlas: atlas_handle.clone(),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(MouseTarget);

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
                LayerType::Floor => {
                    if let Some(floor_sprite) = game_config.random_floor_sprite("cave_floor") {
                        let handle = asset_server.get_handle(&floor_sprite.path);
                        let texture_index = texture_atlas.get_texture_index(&handle).unwrap();
                        commands.spawn(SpriteSheetBundle {
                            transform: Transform {
                                translation: position,
                                scale: Vec3::splat(game_config.tile_scale()),
                                ..default()
                            },
                            sprite: TextureAtlasSprite::new(texture_index),
                            texture_atlas: atlas_handle.clone(),
                            ..default()
                        });
                    }
                }
                LayerType::RoomWall => {
                    if let Some(structure_config) = game_config.structure_config_by_key("room_wall")
                    {
                        if let Some(wall_sprite) = structure_config.max_health_sprite() {
                            let handle = asset_server.get_handle(&wall_sprite.path);
                            let texture_index = texture_atlas.get_texture_index(&handle).unwrap();
                            commands.spawn(SpriteSheetBundle {
                                transform: Transform {
                                    translation: position,
                                    scale: Vec3::splat(game_config.tile_scale()),
                                    ..default()
                                },
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas_handle.clone(),
                                ..default()
                            });
                        }
                    }
                }
                LayerType::RoomFloor => {
                    if let Some(floor_sprite) = game_config.random_floor_sprite("dirt_floor") {
                        let handle = asset_server.get_handle(&floor_sprite.path);
                        let texture_index = texture_atlas.get_texture_index(&handle).unwrap();
                        commands.spawn(SpriteSheetBundle {
                            transform: Transform {
                                translation: position,
                                scale: Vec3::splat(game_config.tile_scale()),
                                ..default()
                            },
                            sprite: TextureAtlasSprite::new(texture_index),
                            texture_atlas: atlas_handle.clone(),
                            ..default()
                        });
                    }
                }
                LayerType::Door => {
                    if let Some(floor_sprite) = game_config.random_floor_sprite("sand_floor") {
                        let handle = asset_server.get_handle(&floor_sprite.path);
                        let texture_index = texture_atlas.get_texture_index(&handle).unwrap();
                        commands.spawn(SpriteSheetBundle {
                            transform: Transform {
                                translation: position,
                                scale: Vec3::splat(game_config.tile_scale()),
                                ..default()
                            },
                            sprite: TextureAtlasSprite::new(texture_index),
                            texture_atlas: atlas_handle.clone(),
                            ..default()
                        });
                    }
                }
                LayerType::OuterWall => {
                    if let Some(structure_config) =
                        game_config.structure_config_by_key("outer_wall")
                    {
                        if let Some(wall_sprite) = structure_config.max_health_sprite() {
                            let handle = asset_server.get_handle(&wall_sprite.path);
                            let texture_index = texture_atlas.get_texture_index(&handle).unwrap();
                            commands.spawn(SpriteSheetBundle {
                                transform: Transform {
                                    translation: position,
                                    scale: Vec3::splat(game_config.tile_scale()),
                                    ..default()
                                },
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas_handle.clone(),
                                ..default()
                            });
                        }
                    }
                }
                LayerType::Rubble => {
                    if let Some(structure_config) = game_config.structure_config_by_key("rubble") {
                        if let Some(wall_sprite) = structure_config.max_health_sprite() {
                            let handle = asset_server.get_handle(&wall_sprite.path);
                            let texture_index = texture_atlas.get_texture_index(&handle).unwrap();
                            commands.spawn(SpriteSheetBundle {
                                transform: Transform {
                                    translation: position,
                                    scale: Vec3::splat(game_config.tile_scale()),
                                    ..default()
                                },
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas_handle.clone(),
                                ..default()
                            });
                        }
                    }
                }
                LayerType::Table => {
                    if let Some(structure_config) = game_config.structure_config_by_key("table") {
                        if let Some(wall_sprite) = structure_config.max_health_sprite() {
                            let handle = asset_server.get_handle(&wall_sprite.path);
                            let texture_index = texture_atlas.get_texture_index(&handle).unwrap();
                            commands.spawn(SpriteSheetBundle {
                                transform: Transform {
                                    translation: position,
                                    scale: Vec3::splat(game_config.tile_scale()),
                                    ..default()
                                },
                                sprite: TextureAtlasSprite::new(texture_index),
                                texture_atlas: atlas_handle.clone(),
                                ..default()
                            });
                        }
                    }
                }
                LayerType::Note => {
                    println!("Note {} {:?}", index, &cell.coordinate());
                }
                LayerType::CommonItem => {
                    println!("common item {} {:?}", index, &cell.coordinate());
                }
                _ => {}
            }
        }
    }

    if let Some(character_config) = game_config.character_config("little_goblin") {
        let goblin_handle = asset_server.get_handle(&character_config.sprite.path);
        let target_index = texture_atlas.get_texture_index(&goblin_handle).unwrap();
        let coordinate = world_coordinate_from_grid(
            top_down_map.entry(),
            game_config.grid_size().get(),
            game_config.tile_size(),
        );
        println!("{:?}", top_down_map.entry());
        commands.spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(coordinate.x, coordinate.y, 10.0),
                scale: Vec3::splat(game_config.tile_scale()),
                ..default()
            },
            sprite: TextureAtlasSprite::new(target_index),
            texture_atlas: atlas_handle,
            ..default()
        });
    }
}
