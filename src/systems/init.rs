use bevy::prelude::{Commands, Query, ResMut, Visibility};
use tdlg::map::{
    cells::Coordinate,
    layers::{LayerType, StructureType},
};

use crate::{
    components::{
        characters::CreatureType,
        jobs::{ExplorationHistory, GlobalAssignmentMode},
        movement::CameraMoveTimer,
        resources::Resource,
        CharacterSpawnable, CharacterSpawns, Map, MapSpawns, SpawnCoordinate, StructureSpawns,
        TdlgSpawnable, World, WorldTickCalculation,
    },
    resources::config::GameConfiguration,
};

fn resource_for_layer(layer_type: &LayerType) -> Option<Resource> {
    match layer_type {
        LayerType::Empty
        | LayerType::Entrance
        | LayerType::Exit
        | LayerType::Item(_)
        | LayerType::Floor(_)
        | LayerType::Note
        | LayerType::Path => None,
        LayerType::Structure(structure_type) => match structure_type {
            StructureType::Rubble | StructureType::Rocks => Some(Resource::Stone(2)),
            StructureType::Boulder => None,
            StructureType::Door => None,
            StructureType::Other => None,
            StructureType::Table => None,
            StructureType::Wall => None,
        },
    }
}

pub fn spawn_starting(
    mut commands: Commands,
    mut game_config: ResMut<GameConfiguration>,
    mut character_spawns_query: Query<&mut CharacterSpawns>,
    mut map_spawns_query: Query<&mut MapSpawns>,
    structure_spawns_query: Query<&StructureSpawns>,
) {
    commands.spawn(World::default());
    commands.spawn(WorldTickCalculation::Running);
    commands.spawn(CameraMoveTimer {
        timer: game_config.movement_timer(),
    });

    commands.spawn(ExplorationHistory::default());

    commands.spawn(GlobalAssignmentMode::manual());

    let top_down_map = game_config.generate_top_down_map();
    let mut tdlg_spawnables: Vec<TdlgSpawnable> = Vec::new();
    for cell in top_down_map.grid().cells() {
        for (index, layer) in cell.layers().iter().enumerate() {
            tdlg_spawnables.push(TdlgSpawnable {
                spawn_coordinate: SpawnCoordinate {
                    coordinate: *cell.coordinate(),
                    z_level: index as f32,
                },
                visibility: Visibility::Hidden,
                resource: resource_for_layer(layer),
                layer_type: *layer,
            });
        }
    }

    let spawnable = CharacterSpawnable {
        spawn_type: CreatureType::Goblin,
        coordinate: SpawnCoordinate {
            coordinate: *top_down_map.entry(),
            z_level: 10.,
        },
    };
    let other_coord =
        Coordinate::from((top_down_map.entry().x() + 1, top_down_map.entry().y() + 1));

    let spawnable_2 = CharacterSpawnable {
        spawn_type: CreatureType::Goblin,
        coordinate: SpawnCoordinate {
            coordinate: other_coord,
            z_level: 10.,
        },
    };
    match character_spawns_query.get_single_mut() {
        Ok(mut character_spawns) => {
            character_spawns.spawnables.push(spawnable);
            character_spawns.spawnables.push(spawnable_2);
        }
        Err(_) => {
            commands.spawn(CharacterSpawns {
                spawnables: vec![spawnable, spawnable_2],
            });
        }
    };

    match map_spawns_query.get_single_mut() {
        Ok(mut map_spawns) => map_spawns.tdlg_spawnables.append(&mut tdlg_spawnables),
        Err(_) => {
            commands.spawn(MapSpawns {
                tdlg_spawnables,
                zone_spawnables: Vec::new(),
            });
        }
    };

    if structure_spawns_query.get_single().is_err() {
        commands.spawn(StructureSpawns {
            spawnables: Vec::new(),
        });
    }

    commands.spawn(Map {
        current: top_down_map,
        tile_size: game_config.tile_size(),
        grid_size: game_config.grid_size().get(),
    });
}
