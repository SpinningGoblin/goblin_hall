use serde::{Deserialize, Serialize};

use super::SingleSprite;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterConfig {
    pub key: String,
    pub sprite: SingleSprite,
}
