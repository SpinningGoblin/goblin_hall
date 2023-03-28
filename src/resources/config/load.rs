use std::io;

use super::{
    game::{GameBasics, GameConfiguration},
    CameraConfig, CharacterConfig, SpriteGroup, StructureConfig, ZonesConfig,
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

#[cfg(feature = "embedded")]
pub fn game_json() -> &'static str {
    include_str!("../../../assets/config/game.json")
}

#[cfg(feature = "embedded")]
pub fn floors_json() -> &'static str {
    include_str!("../../../assets/config/floors.json")
}

#[cfg(feature = "embedded")]
pub fn structures_json() -> &'static str {
    include_str!("../../../assets/config/structures.json")
}

#[cfg(feature = "embedded")]
pub fn characters_json() -> &'static str {
    include_str!("../../../assets/config/characters.json")
}

#[cfg(feature = "embedded")]
pub fn camera_json() -> &'static str {
    include_str!("../../../assets/config/camera.json")
}

#[cfg(feature = "embedded")]
pub fn zones_json() -> &'static str {
    include_str!("../../../assets/config/zones.json")
}

#[cfg(feature = "embedded")]
pub fn load_game_configuration() -> Result<GameConfiguration, LoadError> {
    let basics: GameBasics = serde_json::from_str(game_json())?;
    let floor_sprites: Vec<SpriteGroup> = serde_json::from_str(floors_json())?;
    let structures: Vec<StructureConfig> = serde_json::from_str(structures_json())?;
    let characters: Vec<CharacterConfig> = serde_json::from_str(characters_json())?;
    let camera: CameraConfig = serde_json::from_str(camera_json())?;
    let zones: ZonesConfig = serde_json::from_str(zones_json())?;

    Ok(GameConfiguration::new(
        basics,
        floor_sprites,
        structures,
        characters,
        camera,
    ))
}

#[cfg(not(feature = "embedded"))]
pub fn load_game_configuration() -> Result<GameConfiguration, LoadError> {
    let basics_text = std::fs::read_to_string("./assets/config/game.json")?;
    let basics: GameBasics = serde_json::from_str(&basics_text)?;

    let floor_text = std::fs::read_to_string("./assets/config/floors.json")?;
    let floor_sprites: Vec<SpriteGroup> = serde_json::from_str(&floor_text)?;

    let structures_text = std::fs::read_to_string("./assets/config/structures.json")?;
    let structures: Vec<StructureConfig> = serde_json::from_str(&structures_text)?;

    let characters_text = std::fs::read_to_string("./assets/config/characters.json")?;
    let characters: Vec<CharacterConfig> = serde_json::from_str(&characters_text)?;

    let camera_text = std::fs::read_to_string("./assets/config/camera.json")?;
    let camera: CameraConfig = serde_json::from_str(&camera_text)?;

    let zone_text = std::fs::read_to_string("./assets/config/zones.json")?;
    let zones: ZonesConfig = serde_json::from_str(&zone_text)?;

    Ok(GameConfiguration::new(
        basics,
        floor_sprites,
        structures,
        characters,
        camera,
        zones,
    ))
}
