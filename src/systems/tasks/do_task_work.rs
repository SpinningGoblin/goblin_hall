use bevy::prelude::{info, Commands, Query, Transform, Visibility, With, Without};
use tdlg::map::layers::{LayerType, StructureType};

use crate::components::{
    characters::Character,
    jobs::ExplorationHistory,
    structures::{GridBody, Mineable},
    tasks::{Task, Todo},
    zones::ZoneType,
    Map, MapSpawnable, MapSpawns, SpawnCoordinate, ZoneSpawnable,
};

type NotMineableCharacter = (With<Mineable>, Without<Character>);
type TransformBody = (&'static Transform, &'static GridBody);

pub fn do_task_work(
    mut commands: Commands,
    mut query: Query<(&Character, &mut Transform, &mut Todo)>,
    mineable_query: Query<TransformBody, NotMineableCharacter>,
    mut exploration_history_query: Query<&mut ExplorationHistory>,
    mut map_query: Query<&mut Map>,
    mut map_spawns_query: Query<&mut MapSpawns>,
) {
    let all_query_items = (
        map_query.get_single_mut(),
        exploration_history_query.get_single_mut(),
        map_spawns_query.get_single_mut(),
    );
    let (Ok(mut map), Ok(mut exploration_history), Ok(mut map_spawns)) = all_query_items else {
        return;
    };

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

                            map_spawns.map_spawnables.push(MapSpawnable {
                                layer_type: LayerType::Structure(StructureType::Rubble),
                                spawn_coordinate: SpawnCoordinate {
                                    coordinate: body.center_coordinate,
                                    z_level: transform.translation.z,
                                },
                                visibility: Visibility::Visible,
                            });
                        }
                    }
                }
                Task::ClearExplorationTarget(exploration_target) => {
                    if let Some(entity) = exploration_target.entity {
                        commands.entity(entity).despawn();
                        exploration_target.entity = None;
                    }
                }
                Task::SetupStorageArea(ref mut setup_storage_area) => {
                    setup_storage_area.done = true;
                    map_spawns.zone_spawnables.push(ZoneSpawnable {
                        spawn_coordinate: SpawnCoordinate {
                            coordinate: setup_storage_area.coordinate,
                            z_level: 10.0,
                        },
                        zone_type: ZoneType::StorageArea,
                    })
                }
            };
        }
    }
}
