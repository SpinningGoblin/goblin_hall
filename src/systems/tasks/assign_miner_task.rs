use bevy::prelude::{Commands, Entity, Query, Transform, With};
use tdlg::map::layers::{LayerType, StructureType};

use crate::{
    components::{
        characters::Character,
        jobs::Miner,
        movement::{Path, VisitedPoint},
        structures::{GridBody, Mineable},
        tasks::{MineTask, MiningTarget, WithoutTask},
        GridBox, Map,
    },
    resources::config::grid::{grid_coordinate_from_world, pathfind},
};

type CharacterTransform = (&'static Character, Entity, &'static Transform);
type MinerWithoutTask = (With<Miner>, WithoutTask);

pub fn assign_miner_task(
    mut commands: Commands,
    query: Query<CharacterTransform, MinerWithoutTask>,
    mineable_query: Query<(&Mineable, &GridBody, Entity)>,
    map_query: Query<&Map>,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };

    for character_bundle in query.iter() {
        let (character, entity, transform) = character_bundle;

        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = character.visibility_box(character_coordinate);

        let possible_task = build_miner_task(&visibility_box, &mineable_query, map);

        if let Some(task) = possible_task {
            commands.entity(entity).insert(task);
        }
    }
}

fn is_wall(layer_type: &LayerType) -> bool {
    matches!(*layer_type, LayerType::Structure(StructureType::Wall))
}

fn build_miner_task(
    visibility_box: &GridBox,
    mineable_query: &Query<(&Mineable, &GridBody, Entity)>,
    map: &Map,
) -> Option<MineTask> {
    let mut seen_structures: Vec<(&Mineable, &GridBody, Entity)> = mineable_query
        .iter()
        .filter(|(structure, body, _)| {
            is_wall(&structure.layer_type) && visibility_box.contains(&body.center_coordinate)
        })
        .collect();
    seen_structures
        .sort_by_key(|(_, body, _)| body.center_coordinate.distance(&visibility_box.center));

    seen_structures
        .iter()
        .find_map(|(structure, body, entity)| {
            if body.center_coordinate.distance(&visibility_box.center) <= 1 {
                Some(MineTask {
                    target: MiningTarget {
                        entity: Some(*entity),
                        layer_type: Some(structure.layer_type),
                        coordinate: body.center_coordinate,
                        path: Path {
                            direction: None,
                            points: Vec::new(),
                        },
                    },
                })
            } else {
                pathfind(map, &visibility_box.center, &body.center_coordinate).map(|path| {
                    MineTask {
                        target: MiningTarget {
                            entity: Some(*entity),
                            layer_type: Some(structure.layer_type),
                            coordinate: body.center_coordinate,
                            path: Path {
                                direction: None,
                                points: path
                                    .iter()
                                    .map(|point| VisitedPoint::from(*point))
                                    .collect(),
                            },
                        },
                    }
                })
            }
        })
}
