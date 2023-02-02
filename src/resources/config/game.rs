use std::{num::NonZeroU16, time::Duration};

use bevy::{
    prelude::{Resource, Vec3},
    time::Timer,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tdlg::{generation::Generator, map::TopDownMap};

use super::{
    CameraConfig, MovementConfig, MovementTimer, SingleSprite, SpriteGroup, SpriteTileStats,
    StructureConfig,
};

#[derive(Debug, Resource)]
pub struct GameConfiguration {
    pub basics: GameBasics,
    pub floor_sprites: Vec<SpriteGroup>,
    pub structures: Vec<StructureConfig>,
    generator: Generator,
    movement_timer: MovementTimer,
}

impl GameConfiguration {
    pub fn new(
        basics: GameBasics,
        floor_sprites: Vec<SpriteGroup>,
        structures: Vec<StructureConfig>,
    ) -> Self {
        let generator = tdlg::generation::builder()
            .seed(&basics.grid_generation.seed)
            .grid_size(basics.grid_generation.size)
            .target_number_rooms(basics.grid_generation.target_num_rooms)
            .build();
        Self {
            floor_sprites,
            generator,
            structures,
            movement_timer: basics.movement_timer(),
            basics,
        }
    }

    pub fn tile_size(&self) -> f32 {
        self.basics.tiles.size
    }

    pub fn tile_scale(&self) -> f32 {
        self.basics.tiles.scale
    }

    pub fn grid_size(&self) -> NonZeroU16 {
        self.basics.grid_generation.size
    }

    pub fn generate_top_down_map(&mut self) -> TopDownMap {
        self.generator.generate_top_down_map().unwrap()
    }

    pub fn reset_movement_timer(&mut self) {
        self.movement_timer.0.reset();
    }

    pub fn tick_movement_timer(&mut self, delta: Duration) -> &Timer {
        self.movement_timer.0.tick(delta)
    }

    pub fn camera_movement_modifier(&self) -> f32 {
        self.basics.camera.speed_modifier
    }

    pub fn random_floor_sprite(&self, key: &str) -> Option<&SingleSprite> {
        self.floor_sprites.iter().find_map(|sprite_group| {
            if sprite_group.key.eq(key) && !sprite_group.sprites.is_empty() {
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..sprite_group.sprites.len());
                sprite_group.sprites.get(index)
            } else {
                None
            }
        })
    }

    pub fn structure_config_by_key(&self, key: &str) -> Option<&StructureConfig> {
        self.structures
            .iter()
            .find(|structure_config| structure_config.key.eq(key))
    }

    pub fn initial_camera_scale(&self) -> Vec3 {
        self.basics.camera.initial_camera_scale()
    }

    pub fn zoom_out_level(&self, current: &Vec3) -> Option<Vec3> {
        self.basics.camera.zoom_out_level(current)
    }

    pub fn zoom_in_level(&self, current: &Vec3) -> Option<Vec3> {
        self.basics.camera.zoom_in_level(current)
    }
}

#[derive(Clone, Debug, Deserialize, Resource, Serialize)]
pub struct GameBasics {
    tiles: SpriteTileStats,
    grid_generation: GridGeneration,
    movement: MovementConfig,
    camera: CameraConfig,
}

impl GameBasics {
    pub fn movement_timer(&self) -> MovementTimer {
        self.movement.movement_timer()
    }
}

#[derive(Clone, Debug, Deserialize, Resource, Serialize)]
pub struct GridGeneration {
    size: NonZeroU16,
    target_num_rooms: NonZeroU16,
    seed: String,
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;

    use bevy::prelude::Vec3;

    use crate::resources::config::{
        game::GridGeneration, CameraConfig, MovementConfig, MovementTimerConfig, SpriteTileStats,
        ZoomLevel,
    };

    use super::GameBasics;

    #[test]
    fn serialize_json() {
        let basics = GameBasics {
            tiles: SpriteTileStats {
                size: 32.0,
                scale: 4.0,
            },
            grid_generation: GridGeneration {
                size: NonZeroU16::new(20).unwrap(),
                target_num_rooms: NonZeroU16::new(20).unwrap(),
                seed: "test".to_string(),
            },
            movement: MovementConfig {
                timer: MovementTimerConfig { wait_time: 0.2 },
            },
            camera: CameraConfig {
                initial_zoom_level: 1,
                speed_modifier: 4.0,
                zoom_levels: vec![ZoomLevel {
                    order: 1,
                    scale: Vec3::splat(1.0),
                }],
            },
        };

        let serialized = serde_json::to_string(&basics).unwrap();
        assert_eq!("{\"tiles\":{\"size\":32.0,\"scale\":4.0},\"grid_generation\":{\"size\":20,\"target_num_rooms\":20,\"seed\":\"test\"},\"movement\":{\"timer\":{\"wait_time\":0.2}},\"camera\":{\"initial_zoom_level\":1,\"speed_modifier\":4.0,\"zoom_levels\":[{\"order\":1,\"scale\":[1.0,1.0,1.0]}]}}", &serialized);
    }
}
