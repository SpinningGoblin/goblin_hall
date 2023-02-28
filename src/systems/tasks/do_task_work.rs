use bevy::{
    prelude::{
        default, info, AssetServer, Commands, Query, Res, Transform, Vec3, Visibility, With,
        Without,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};
use tdlg::map::layers::{LayerType, StructureType};

use crate::{
    components::{
        characters::Character,
        jobs::ExplorationHistory,
        structures::{Body, Mineable, Structure},
        tasks::{Task, Todo},
        Map, World,
    },
    resources::{config::GameConfiguration, sprites::Atlas},
};

pub fn do_task_work(
    mut commands: Commands,
    mut query: Query<(&Character, &mut Transform, &mut Todo)>,
    mineable_query: Query<(&Transform, &Body), (With<Mineable>, Without<Character>)>,
    mut exploration_history_query: Query<&mut ExplorationHistory>,
    world_query: Query<&World>,
    mut map_query: Query<&mut Map>,
    atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfiguration>,
) {
    if world_query.is_empty() || exploration_history_query.is_empty() || map_query.is_empty() {
        return;
    }

    let world = world_query.single();
    if !world.tick_just_finished {
        return;
    }

    let mut map = map_query.single_mut();

    let mut exploration_history = exploration_history_query.single_mut();
    for character_bundle in query.iter_mut() {
        let (_, mut transform, mut todo) = character_bundle;

        if let Some(current_task) = todo.tasks.iter_mut().find(|task| !task.is_complete()) {
            match current_task {
                Task::Walk(ref mut path) => {
                    info!("Walking");
                    if let Some(visited_point) = path
                        .points
                        .iter_mut()
                        .find(|visited_point| !visited_point.visited)
                    {
                        transform.translation.x = visited_point.point.x;
                        transform.translation.y = visited_point.point.y;
                        visited_point.visited = true;
                        exploration_history.points.push(visited_point.point);
                    }
                }
                Task::Mine(mining_target) => {
                    // TODO: I need to spawn rocks and they can be... rockables? RockProviders?
                    // Unsure exactly but they will let characters pick up rocks so they can be
                    // stockpiled, and eventually used for building.
                    if let Some(entity) = mining_target.entity {
                        if let Ok((transform, body)) = mineable_query.get(entity) {
                            commands.entity(entity).despawn();
                            mining_target.entity = None;

                            if let Some(layer) = &mining_target.layer_type {
                                map.current
                                    .grid_mut()
                                    .remove_layer(&mining_target.coordinate, *layer);
                            }

                            if let Some(structure_config) =
                                game_config.structure_config_by_key("rubble")
                            {
                                if let Some(wall_sprite) = structure_config.max_health_sprite() {
                                    let handle = asset_server.get_handle(&wall_sprite.path);
                                    let texture_index =
                                        atlas.texture_atlas.get_texture_index(&handle).unwrap();
                                    commands
                                        .spawn(SpriteSheetBundle {
                                            transform: Transform {
                                                translation: transform.translation,
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
                                            cell_center: transform.translation.truncate(),
                                            underground: false,
                                            center_coordinate: body.center_coordinate,
                                        });
                                }
                            }
                        }
                    }
                }
                Task::ClearExplorationTarget(exploration_target) => {
                    if let Some(entity) = exploration_target.entity {
                        commands.entity(entity).despawn();
                        exploration_target.entity = None;
                    }
                }
            };
        }
    }
}
