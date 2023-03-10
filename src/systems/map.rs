use bevy::prelude::{Commands, Query, ResMut, Visibility};

use crate::{
    components::{
        jobs::ExplorationHistory, movement::CameraMoveTimer, CharacterSpawnable, CharacterSpawns,
        Map, MapSpawnable, MapSpawns, SpawnCoordinate, World,
    },
    resources::config::GameConfiguration,
};

pub fn spawn_starting(
    mut commands: Commands,
    mut game_config: ResMut<GameConfiguration>,
    mut character_spawns_query: Query<&mut CharacterSpawns>,
    mut map_spawns_query: Query<&mut MapSpawns>,
) {
    commands.spawn(World::default());
    commands.spawn(CameraMoveTimer {
        timer: game_config.movement_timer(),
    });

    commands.spawn(ExplorationHistory::default());

    let top_down_map = game_config.generate_top_down_map();
    let mut map_spawnables: Vec<MapSpawnable> = Vec::new();
    for cell in top_down_map.grid().cells() {
        for (index, layer) in cell.layers().iter().enumerate() {
            map_spawnables.push(MapSpawnable {
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
        Ok(mut map_spawns) => map_spawns.spawnables.append(&mut map_spawnables),
        Err(_) => {
            commands.spawn(MapSpawns {
                spawnables: map_spawnables,
            });
        }
    };

    commands.spawn(Map {
        current: top_down_map,
        tile_size: game_config.tile_size(),
        grid_size: game_config.grid_size().get(),
    });
}
