use bevy::prelude::{info, Commands, Query, Transform, Visibility, With, Without};
use tdlg::map::layers::{LayerType, StructureType};

use crate::{
    components::{
        characters::Character,
        jobs::ExplorationHistory,
        structures::{GridBody, Mineable},
        tasks::Task,
        Map, MapSpawns, SpawnCoordinate, StructureSpawnable, StructureSpawns, TdlgSpawnable,
    },
    utils,
};

type NotMineableCharacter = (With<Mineable>, Without<Character>);
type TransformBody = (&'static Transform, &'static GridBody);

pub fn do_task_work(
    mut commands: Commands,
    mut query: Query<(&Character, &mut Transform, &mut Task)>,
    mineable_query: Query<TransformBody, NotMineableCharacter>,
    mut exploration_history_query: Query<&mut ExplorationHistory>,
    mut map_query: Query<&mut Map>,
    mut map_spawns_query: Query<&mut MapSpawns>,
    mut structure_spawns_query: Query<&mut StructureSpawns>,
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
        let (_, mut transform, mut task) = character_bundle;

        match task.as_mut() {
            Task::Walk(ref mut path) => {
                info!("Walking");
                utils::movement::visit_next_point(
                    path,
                    transform.as_mut(),
                    exploration_history.as_mut(),
                );
            }
            Task::Mine(mining_target) => {
                // TODO: I need to spawn rocks and they can be... rockables? RockProviders?
                // Unsure exactly but they will let characters pick up rocks so they can be
                // stockpiled, and eventually used for building.
                if mining_target.path_incomplete() {
                    utils::movement::visit_next_point(
                        &mut mining_target.path,
                        transform.as_mut(),
                        exploration_history.as_mut(),
                    );
                } else if let Some(entity) = mining_target.entity {
                    if let Ok((transform, body)) = mineable_query.get(entity) {
                        commands.entity(entity).despawn();
                        mining_target.entity = None;

                        if let Some(layer) = &mining_target.layer_type {
                            map.current
                                .grid_mut()
                                .remove_layer(&mining_target.coordinate, *layer);
                        }

                        map_spawns.tdlg_spawnables.push(TdlgSpawnable {
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
                if exploration_target.path_incomplete() {
                    utils::movement::visit_next_point(
                        &mut exploration_target.path,
                        transform.as_mut(),
                        exploration_history.as_mut(),
                    );
                } else if let Some(entity) = exploration_target.entity {
                    info!("despawning the exploration target");
                    commands.entity(entity).despawn();
                    exploration_target.entity = None;
                }
            }
            Task::SetupStorageArea(ref mut setup_storage_area) => {
                if setup_storage_area.path_incomplete() {
                    utils::movement::visit_next_point(
                        &mut setup_storage_area.path,
                        transform.as_mut(),
                        exploration_history.as_mut(),
                    );
                } else if let Some(setup_entity) = setup_storage_area.entity {
                    commands.entity(setup_entity).despawn();
                    setup_storage_area.entity = None;
                } else {
                    setup_storage_area.done = true;
                    if let Ok(mut structure_spawns) = structure_spawns_query.get_single_mut() {
                        structure_spawns.spawnables.push(StructureSpawnable {
                            spawn_type: crate::components::StructureSpawnType::StorageArea,
                            spawn_coordinate: SpawnCoordinate {
                                coordinate: setup_storage_area.coordinate,
                                z_level: 20.,
                            },
                            visibility: Visibility::Inherited,
                        });
                    }
                }
            }
        };
    }
}
