use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use tdlg::map::layers::LayerType;

#[derive(Clone, Debug, Deserialize, Resource, Serialize)]
pub struct SpriteTileStats {
    pub size: f32,
    pub scale: f32,
}

#[derive(Clone, Debug, Deserialize, Resource, Serialize)]
pub struct SpriteGroup {
    pub key: String,
    pub layer_type: SpriteLayerType,
    pub sprites: Vec<SingleSprite>,
    #[serde(default)]
    pub tile_stats: Option<SpriteTileStats>,
}

#[derive(Clone, Debug, Deserialize, Resource, Serialize)]
pub struct SingleSprite {
    pub key: String,
    pub path: String,
    #[serde(default)]
    pub tile_stats: Option<SpriteTileStats>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Resource, Serialize)]
pub enum SpriteLayerType {
    TopDownMap(LayerType),
}

#[cfg(test)]
mod tests {
    use tdlg::map::layers::LayerType;

    use super::{SingleSprite, SpriteGroup, SpriteLayerType};

    #[test]
    fn serialize() {
        let single_sprite = SingleSprite {
            key: "floor".to_string(),
            path: "assets/floor/purple_1.png".to_string(),
            tile_stats: None,
        };
        let serialized = serde_json::to_string(&single_sprite).unwrap();
        assert_eq!(
            "{\"key\":\"floor\",\"path\":\"assets/floor/purple_1.png\",\"tile_stats\":null}",
            serialized
        );
    }

    #[test]
    fn serialize_group() {
        let group = SpriteGroup {
            key: "purple floor".to_string(),
            layer_type: SpriteLayerType::TopDownMap(LayerType::Floor),
            sprites: vec![SingleSprite {
                key: "floor".to_string(),
                path: "assets/floor/purple_1.png".to_string(),
                tile_stats: None,
            }],
            tile_stats: None,
        };
        let serialized = serde_json::to_string(&group).unwrap();
        assert_eq!("{\"key\":\"purple floor\",\"layer_type\":{\"TopDownMap\":\"floor\"},\"sprites\":[{\"key\":\"floor\",\"path\":\"assets/floor/purple_1.png\",\"tile_stats\":null}],\"tile_stats\":null}", serialized);
    }

    #[test]
    fn deserialize() {
        let serialized = r#"
        {
            "key": "floor",
            "path": "assets/floor/purple_1.png"
        }
        "#;
        let deserialized: SingleSprite = serde_json::from_str(serialized).unwrap();
        assert_eq!("floor", &deserialized.key);
    }

    #[test]
    fn deserialize_group() {
        let serialized = r#"
        {
            "key": "purple floor",
            "layer_type": {
                "TopDownMap": "floor"
            },
            "sprites": [
                {
                    "key": "purple floor 1",
                    "path": "assets/floor/purple_1.png"
                },
                {
                    "key": "purple floor 2",
                    "path": "assets/floor/purple_2.png"
                }
            ]
        }
        "#;
        let deserialized: SpriteGroup = serde_json::from_str(serialized).unwrap();
        assert_eq!(2, deserialized.sprites.len());
        assert_eq!(
            SpriteLayerType::TopDownMap(LayerType::Floor),
            deserialized.layer_type
        );
    }
}
