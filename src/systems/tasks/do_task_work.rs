use bevy::prelude::{info, Commands, Entity, Query, Transform, Visibility, With, Without};
use tdlg::map::layers::{LayerType, StructureType};

use crate::{
    components::{
        characters::Character,
        jobs::{Builder, ExplorationHistory, Explorer, Miner, PreviousExplorations},
        structures::{GridBody, Mineable},
        tasks::{ClearExplorationTargetTask, MineTask, SetupStorageAreaTask, WalkTask},
        Map, MapSpawns, SpawnCoordinate, StructureSpawnable, StructureSpawns, TdlgSpawnable,
    },
    utils,
};

type NotMineableCharacter = (With<Mineable>, Without<Character>);
type TransformBody = (&'static Transform, &'static GridBody);

pub fn do_walk_work(
    mut commands: Commands,
    mut query: Query<(
        &Character,
        &mut Transform,
        &mut WalkTask,
        Entity,
        &mut PreviousExplorations,
    )>,
    mut exploration_history_query: Query<&mut ExplorationHistory>,
) {
    let Ok(mut exploration_history) = exploration_history_query.get_single_mut() else {
        return;
    };

    for character_bundle in query.iter_mut() {
        let (_, mut transform, mut walk_task, entity, mut previous_explorations) = character_bundle;

        utils::movement::visit_next_point(
            &mut walk_task.path,
            transform.as_mut(),
            exploration_history.as_mut(),
        );

        if walk_task.is_complete() {
            if let Some(direction) = &walk_task.path.direction {
                previous_explorations.direction = Some(*direction);
            }

            commands
                .entity(entity)
                .remove::<WalkTask>()
                .remove::<Explorer>();
        }
    }
}

pub fn do_mining_work(
    mut commands: Commands,
    mut query: Query<(&mut MineTask, &mut Transform, &Character, Entity)>,
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
        let (mut mine_task, mut transform, _, entity) = character_bundle;
        // TODO: I need to spawn rocks and they can be... rockables? RockProviders?
        // Unsure exactly but they will let characters pick up rocks so they can be
        // stockpiled, and eventually used for building.
        if mine_task.target.path_incomplete() {
            utils::movement::visit_next_point(
                &mut mine_task.target.path,
                transform.as_mut(),
                exploration_history.as_mut(),
            );
        } else if let Some(entity) = mine_task.target.entity {
            if let Ok((transform, body)) = mineable_query.get(entity) {
                commands.entity(entity).despawn();
                mine_task.target.entity = None;

                if let Some(layer) = &mine_task.target.layer_type {
                    map.current
                        .grid_mut()
                        .remove_layer(&mine_task.target.coordinate, *layer);
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

        if mine_task.is_complete() {
            commands
                .entity(entity)
                .remove::<MineTask>()
                .remove::<Miner>();
        }
    }
}

pub fn do_clear_exploration_work(
    mut commands: Commands,
    mut query: Query<(
        &Character,
        &mut Transform,
        &mut ClearExplorationTargetTask,
        Entity,
    )>,
    mut exploration_history_query: Query<&mut ExplorationHistory>,
) {
    let Ok(mut exploration_history) = exploration_history_query.get_single_mut() else {
        return;
    };

    for character_bundle in query.iter_mut() {
        let (_, mut transform, mut clear_expliration_target, entity) = character_bundle;

        if clear_expliration_target.target.path_incomplete() {
            // TODO: I might want to switch up how I'm using the exploration history.
            // I probably do not want all of my systems using and modifying it directly, which
            // would limit their parallelization.
            utils::movement::visit_next_point(
                &mut clear_expliration_target.target.path,
                transform.as_mut(),
                exploration_history.as_mut(),
            );
        } else if let Some(entity) = clear_expliration_target.target.entity {
            info!("despawning the exploration target");
            commands.entity(entity).despawn();
            clear_expliration_target.target.entity = None;
        }

        if clear_expliration_target.is_complete() {
            commands
                .entity(entity)
                .remove::<Explorer>()
                .remove::<ClearExplorationTargetTask>();
        }
    }
}

pub fn do_setup_storage_work(
    mut commands: Commands,
    mut query: Query<(
        &Character,
        &mut Transform,
        &mut SetupStorageAreaTask,
        Entity,
    )>,
    mut exploration_history_query: Query<&mut ExplorationHistory>,
    mut structure_spawns_query: Query<&mut StructureSpawns>,
) {
    let Ok(mut exploration_history) = exploration_history_query.get_single_mut() else {
        return;
    };

    for character_bundle in query.iter_mut() {
        let (_, mut transform, mut task, entity) = character_bundle;

        if task.setup_area.path_incomplete() {
            utils::movement::visit_next_point(
                &mut task.setup_area.path,
                transform.as_mut(),
                exploration_history.as_mut(),
            );
        } else if let Some(setup_entity) = task.setup_area.entity {
            commands.entity(setup_entity).despawn();
            task.setup_area.entity = None;
        } else {
            task.setup_area.done = true;
            if let Ok(mut structure_spawns) = structure_spawns_query.get_single_mut() {
                structure_spawns.spawnables.push(StructureSpawnable {
                    spawn_type: crate::components::StructureSpawnType::StorageArea,
                    spawn_coordinate: SpawnCoordinate {
                        coordinate: task.setup_area.coordinate,
                        z_level: 20.,
                    },
                    visibility: Visibility::Inherited,
                });
            }
        }

        if task.is_complete() {
            commands
                .entity(entity)
                .remove::<SetupStorageAreaTask>()
                .remove::<Builder>();
        }
    }
}
