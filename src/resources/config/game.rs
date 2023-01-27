use std::num::NonZeroU16;

use bevy::prelude::Resource;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tdlg::{generation::Generator, map::TopDownMap};

use super::{SingleSprite, SpriteGroup, SpriteTileStats, StructureConfig};

#[derive(Debug, Resource)]
pub struct GameConfiguration {
    pub basics: GameBasics,
    pub floor_sprites: Vec<SpriteGroup>,
    pub structures: Vec<StructureConfig>,
    generator: Generator,
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
            basics,
            floor_sprites,
            generator,
            structures,
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
}

#[derive(Clone, Debug, Deserialize, Resource, Serialize)]
pub struct GameBasics {
    tiles: SpriteTileStats,
    grid_generation: GridGeneration,
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

    use crate::resources::config::{game::GridGeneration, SpriteTileStats};

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
        };

        let serialized = serde_json::to_string(&basics).unwrap();
        assert_eq!("{\"tiles\":{\"size\":32.0,\"scale\":4.0},\"grid_generation\":{\"size\":20,\"target_num_rooms\":20,\"seed\":\"test\"}}", &serialized);
    }
}
