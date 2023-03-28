use serde::{Deserialize, Serialize};

use super::SingleSprite;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZoneConfig {
    pub key: String,
    pub target: SingleSprite,
    pub overlay: SingleSprite,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZonesConfig {
    pub zones: Vec<ZoneConfig>,
}
