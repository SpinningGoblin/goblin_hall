use serde::{Deserialize, Serialize};

use super::{SingleSprite, SpriteLayerType};

#[derive(Debug, Deserialize, Serialize)]
pub struct StructureConfig {
    pub key: String,
    pub layer_type: SpriteLayerType,
    pub health: u32,
    pub health_configs: Vec<HealthConfig>,
}

impl StructureConfig {
    pub fn max_health_sprite(&self) -> Option<&SingleSprite> {
        let mut sorted: Vec<&HealthConfig> = self.health_configs.iter().collect();
        sorted.sort_by_key(|health_config| health_config.health_range.max);
        sorted.last().map(|health_config| health_config.sprite())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthConfig {
    pub sprite: SingleSprite,
    pub health_range: HealthRange,
    #[serde(default)]
    pub can_be_broken: bool,
    #[serde(default)]
    pub can_be_walked_on: bool,
}

impl HealthConfig {
    pub fn sprite(&self) -> &SingleSprite {
        &self.sprite
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthRange {
    pub min: i32,
    pub max: i32,
}

#[cfg(test)]
mod tests {
    use tdlg::map::cells::LayerType;

    use crate::resources::config::{SingleSprite, SpriteLayerType};

    use super::{HealthConfig, HealthRange, StructureConfig};

    #[test]
    fn serialize() {
        let config = StructureConfig {
            key: "wall".to_lowercase(),
            layer_type: SpriteLayerType::TopDownMap(LayerType::RoomWall),
            health: 22,
            health_configs: vec![HealthConfig {
                sprite: SingleSprite {
                    key: "basic".to_string(),
                    path: "/some/path/sprite.png".to_string(),
                },
                health_range: HealthRange { min: 0, max: 22 },
                can_be_broken: false,
                can_be_walked_on: false,
            }],
        };

        let serialized = serde_json::to_string(&config).unwrap();
        assert_eq!("{\"key\":\"wall\",\"layer_type\":{\"TopDownMap\":\"room_wall\"},\"health\":22,\"health_configs\":[{\"sprite\":{\"key\":\"basic\",\"path\":\"/some/path/sprite.png\"},\"health_range\":{\"min\":0,\"max\":22},\"can_be_broken\":false,\"can_be_walked_on\":false}]}", &serialized);
    }
}
