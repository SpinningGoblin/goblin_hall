use bevy::{
    prelude::{default, AssetServer, Commands, Query, Res, Transform, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::{
    components::{
        characters::{Character, CreatureType, ResourceInventory},
        jobs::{JobPriority, PreviousExplorations},
        CharacterSpawns, Map,
    },
    resources::{
        config::{grid::world_coordinate_from_grid, GameConfiguration},
        sprites::Atlas,
    },
};

pub fn characters(
    mut commands: Commands,
    character_spawns_query: Query<&CharacterSpawns>,
    atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfiguration>,
    map_query: Query<&Map>,
) {
    let (Ok(character_spawns), Ok(map)) = (character_spawns_query.get_single(), map_query.get_single()) else {
        return;
    };

    for spawnable in character_spawns.spawnables.iter() {
        let coordinate = world_coordinate_from_grid(
            &spawnable.coordinate.coordinate,
            map.grid_size,
            map.tile_size,
        );

        match spawnable.spawn_type {
            CreatureType::Goblin => {
                if let Some(character_config) = game_config.character_config("little_goblin") {
                    let goblin_handle = asset_server.get_handle(&character_config.sprite.path);
                    let target_index = atlas
                        .texture_atlas
                        .get_texture_index(&goblin_handle)
                        .unwrap();
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    coordinate.x,
                                    coordinate.y,
                                    spawnable.coordinate.z_level,
                                ),
                                scale: Vec3::splat(game_config.tile_scale()),
                                ..default()
                            },
                            sprite: TextureAtlasSprite::new(target_index),
                            texture_atlas: atlas.atlas_handle.clone(),
                            ..default()
                        })
                        .insert(Character {
                            visibility: character_config.visibility,
                        })
                        .insert(ResourceInventory::default())
                        .insert(PreviousExplorations::default())
                        .insert(JobPriority::default());
                }
            }
        }
    }
}
