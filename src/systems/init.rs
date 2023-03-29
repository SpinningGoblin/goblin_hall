use bevy::prelude::{Commands, Query, ResMut, Visibility};

use crate::{
    components::{
        jobs::ExplorationHistory, movement::CameraMoveTimer, CharacterSpawnable, CharacterSpawns,
        Map, MapSpawns, SpawnCoordinate, StructureSpawns, TdlgSpawnable, World,
    },
    resources::config::GameConfiguration,
};

pub fn spawn_starting(
    mut commands: Commands,
    mut game_config: ResMut<GameConfiguration>,
    mut character_spawns_query: Query<&mut CharacterSpawns>,
    mut map_spawns_query: Query<&mut MapSpawns>,
    structure_spawns_query: Query<&StructureSpawns>,
) {
    commands.spawn(World::default());
    commands.spawn(CameraMoveTimer {
        timer: game_config.movement_timer(),
    });

    commands.spawn(ExplorationHistory::default());

    let top_down_map = game_config.generate_top_down_map();
    let mut tdlg_spawnables: Vec<TdlgSpawnable> = Vec::new();
    for cell in top_down_map.grid().cells() {
        for (index, layer) in cell.layers().iter().enumerate() {
            tdlg_spawnables.push(TdlgSpawnable {
                layer_type: *layer,
                spawn_coordinate: SpawnCoordinate {
                    coordinate: *cell.coordinate(),
                    z_level: index as f32,
                },
                visibility: Visibility::Hidden,
            });
        }
    }

    let spawnable = CharacterSpawnable {
        spawn_type: crate::components::characters::CreatureType::Goblin,
        coordinate: SpawnCoordinate {
            coordinate: *top_down_map.entry(),
            z_level: 10.,
        },
    };
    match character_spawns_query.get_single_mut() {
        Ok(mut character_spawns) => character_spawns.spawnables.push(spawnable),
        Err(_) => {
            commands.spawn(CharacterSpawns {
                spawnables: vec![spawnable],
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

    if let Err(_) = structure_spawns_query.get_single() {
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
