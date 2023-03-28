use std::num::NonZeroU16;

use bevy::prelude::{Resource, Vec3};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tdlg::{generation::Generator, map::TopDownMap};

use super::{
    timers::WorldTickTimerConfig, CameraConfig, CharacterConfig, MovementConfig, MovementTimer,
    SingleSprite, SpriteGroup, SpriteTileStats, StructureConfig, WorldTickTimer, ZoneConfig,
    ZonesConfig,
};

#[derive(Debug, Resource)]
pub struct GameConfiguration {
    pub basics: GameBasics,
    pub floor_sprites: Vec<SpriteGroup>,
    pub structures: Vec<StructureConfig>,
    pub characters: Vec<CharacterConfig>,
    pub camera: CameraConfig,
    pub zones_config: ZonesConfig,
    generator: Generator,
}

impl GameConfiguration {
    pub fn new(
        basics: GameBasics,
        floor_sprites: Vec<SpriteGroup>,
        structures: Vec<StructureConfig>,
        characters: Vec<CharacterConfig>,
        camera: CameraConfig,
        zones: ZonesConfig,
    ) -> Self {
        let generator = tdlg::generation::builder()
            .seed(&basics.grid_generation.seed)
            .grid_size(basics.grid_generation.size)
            .target_number_rooms(basics.grid_generation.target_num_rooms)
            .include_outer_wall(false)
            .build();
        Self {
            floor_sprites,
            generator,
            structures,
            characters,
            camera,
            basics,
            zones_config: zones,
        }
    }

    pub fn movement_timer(&self) -> MovementTimer {
        self.basics.movement_timer()
    }

    pub fn world_timer(&self) -> WorldTickTimer {
        self.basics.world_timer()
    }

    pub fn character_config(&self, key: &str) -> Option<&CharacterConfig> {
        self.characters.iter().find(|config| config.key.eq(key))
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

    pub fn camera_movement_modifier(&self, current_zoom: &Vec3) -> f32 {
        self.camera
            .zoom_level(current_zoom)
            .and_then(|zoom_level| zoom_level.speed_modifier)
            .unwrap_or(self.camera.speed_modifier)
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
        self.camera.initial_camera_scale()
    }

    pub fn zoom_out_level(&self, current: &Vec3) -> Option<Vec3> {
        self.camera.zoom_out_level(current)
    }

    pub fn zoom_in_level(&self, current: &Vec3) -> Option<Vec3> {
        self.camera.zoom_in_level(current)
    }

    pub fn zone_config(&self, key: &str) -> Option<&ZoneConfig> {
        self.zones_config.zones.iter().find(|zone| zone.key.eq(key))
    }
}

#[derive(Clone, Debug, Deserialize, Resource, Serialize)]
pub struct GameBasics {
    tiles: SpriteTileStats,
    grid_generation: GridGeneration,
    movement: MovementConfig,
    zone: SingleSprite,
    world: WorldTickTimerConfig,
}

impl GameBasics {
    pub fn movement_timer(&self) -> MovementTimer {
        self.movement.movement_timer()
    }

    pub fn world_timer(&self) -> WorldTickTimer {
        self.world.timer()
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

    use crate::resources::config::{
        game::GridGeneration, timers::WorldTickTimerConfig, MovementConfig, MovementTimerConfig,
        SingleSprite, SpriteTileStats,
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
            zone: SingleSprite {
                key: "zone".to_string(),
                path: "zone.png".to_string(),
                tile_stats: None,
            },
            world: WorldTickTimerConfig { wait_time: 0.4 },
        };

        let serialized = serde_json::to_string(&basics).unwrap();
        assert_eq!("{\"tiles\":{\"size\":32.0,\"scale\":4.0},\"grid_generation\":{\"size\":20,\"target_num_rooms\":20,\"seed\":\"test\"},\"movement\":{\"timer\":{\"wait_time\":0.2}},\"mouse_target\":{\"key\":\"target\",\"path\":\"outline.png\",\"tile_stats\":null},\"zone\":{\"key\":\"zone\",\"path\":\"zone.png\",\"tile_stats\":null},\"world\":{\"wait_time\":0.4}}", &serialized);
    }
}
