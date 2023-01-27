use std::io;

use super::{
    game::{GameBasics, GameConfiguration},
    SpriteGroup, StructureConfig,
};

#[derive(Debug)]
pub struct LoadError {
    pub message: String,
}

impl From<io::Error> for LoadError {
    fn from(value: io::Error) -> Self {
        LoadError {
            message: value.to_string(),
        }
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(value: serde_json::Error) -> Self {
        LoadError {
            message: value.to_string(),
        }
    }
}

pub fn load_game_configuration() -> Result<GameConfiguration, LoadError> {
    let basics_text = std::fs::read_to_string("./assets/config/game.json")?;
    let basics: GameBasics = serde_json::from_str(&basics_text)?;

    let floor_text = std::fs::read_to_string("./assets/config/floors.json")?;
    let floor_sprites: Vec<SpriteGroup> = serde_json::from_str(&floor_text)?;

    let structures_text = std::fs::read_to_string("./assets/config/structures.json")?;
    let structures: Vec<StructureConfig> = serde_json::from_str(&structures_text)?;

    Ok(GameConfiguration::new(basics, floor_sprites, structures))
}
