use bevy::prelude::{Commands, Entity, Query, Transform, Visibility, With};
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
    mut mineable_query: Query<(&mut Mineable, &GridBody, &Visibility, Entity)>,
    map_query: Query<&Map>,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };

    let mut used_mineables: Vec<Entity> = Vec::new();
    for character_bundle in query.iter() {
        let (character, entity, transform) = character_bundle;

        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = character.visibility_box(character_coordinate);

        let possible_task = build_miner_task(
            &visibility_box,
            &mut mineable_query,
            map,
            &mut used_mineables,
        );

        if let Some(task) = possible_task {
            commands.entity(entity).insert(task);
        } else {
            commands.entity(entity).remove::<Miner>();
        }
    }
}

fn is_wall(layer_type: &LayerType) -> bool {
    matches!(*layer_type, LayerType::Structure(StructureType::Wall))
}

fn build_miner_task(
    visibility_box: &GridBox,
    mineable_query: &mut Query<(&mut Mineable, &GridBody, &Visibility, Entity)>,
    map: &Map,
    used_mineables: &mut Vec<Entity>,
) -> Option<MineTask> {
    let possible_mineable = mineable_query
        .iter_mut()
        .filter(|(mineable, _, visibility, entity)| {
            is_wall(&mineable.layer_type)
                && !mineable.targeted
                && !used_mineables.contains(entity)
                && matches!(visibility, Visibility::Visible | Visibility::Inherited)
        })
        .min_by_key(|(_, body, _, _)| body.center_coordinate.distance(&visibility_box.center));

    if let Some((mut mineable, body, _, entity)) = possible_mineable {
        mineable.targeted = true;
        used_mineables.push(entity);
        if body.center_coordinate.distance(&visibility_box.center) <= 1 {
            Some(MineTask {
                target: MiningTarget {
                    entity: Some(entity),
                    coordinate: body.center_coordinate,
                    path: Path {
                        direction: None,
                        points: Vec::new(),
                    },
                },
            })
        } else {
            pathfind(map, &visibility_box.center, &body.center_coordinate).map(|path| MineTask {
                target: MiningTarget {
                    entity: Some(entity),
                    coordinate: body.center_coordinate,
                    path: Path {
                        direction: None,
                        points: path
                            .iter()
                            .map(|point| VisitedPoint::from(*point))
                            .collect(),
                    },
                },
            })
        }
    } else {
        None
    }
}
