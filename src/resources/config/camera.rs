use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraConfig {
    pub initial_zoom_level: u16,
    pub speed_modifier: f32,
    pub zoom_levels: Vec<ZoomLevel>,
}

impl CameraConfig {
    pub fn zoom_level(&self, scale: &Vec3) -> Option<&ZoomLevel> {
        self.zoom_levels
            .iter()
            .find(|zoom_level| zoom_level.scale.eq(scale))
    }

    pub fn initial_camera_scale(&self) -> Vec3 {
        self.zoom_levels
            .iter()
            .find(|zoom_level| zoom_level.order == self.initial_zoom_level)
            .map(|zoom_level| zoom_level.scale)
            .unwrap_or(Vec3::splat(1.0))
    }

    pub fn zoom_out_level(&self, current: &Vec3) -> Option<Vec3> {
        let max_order = self
            .zoom_levels
            .iter()
            .max_by_key(|zoom_level| zoom_level.order)
            .map(|zoom_level| zoom_level.order)
            .unwrap_or_default();
        if let Some(current_order) = self
            .zoom_levels
            .iter()
            .find(|zoom_level| zoom_level.scale.eq(current))
            .map(|zoom_level| zoom_level.order)
        {
            if current_order == max_order {
                self.zoom_levels
                    .iter()
                    .min_by_key(|zoom_level| zoom_level.order)
                    .map(|zoom_level| zoom_level.scale)
            } else {
                self.zoom_levels
                    .iter()
                    .find(|zoom_level| zoom_level.order == current_order + 1)
                    .map(|zoom_level| zoom_level.scale)
            }
        } else {
            None
        }
    }

    pub fn zoom_in_level(&self, current: &Vec3) -> Option<Vec3> {
        let min_order = self
            .zoom_levels
            .iter()
            .min_by_key(|zoom_level| zoom_level.order)
            .map(|zoom_level| zoom_level.order)
            .unwrap_or_default();
        if let Some(current_order) = self
            .zoom_levels
            .iter()
            .find(|zoom_level| zoom_level.scale.eq(current))
            .map(|zoom_level| zoom_level.order)
        {
            if current_order == min_order {
                self.zoom_levels
                    .iter()
                    .max_by_key(|zoom_level| zoom_level.order)
                    .map(|zoom_level| zoom_level.scale)
            } else {
                self.zoom_levels
                    .iter()
                    .find(|zoom_level| zoom_level.order == current_order - 1)
                    .map(|zoom_level| zoom_level.scale)
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZoomLevel {
    pub order: u16,
    pub scale: Vec3,
    #[serde(default)]
    pub speed_modifier: Option<f32>,
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec3;

    use super::{CameraConfig, ZoomLevel};

    #[test]
    fn serialize() {
        let config = CameraConfig {
            initial_zoom_level: 1,
            speed_modifier: 4.0,
            zoom_levels: vec![ZoomLevel {
                order: 1,
                scale: Vec3::splat(1.0),
                speed_modifier: None,
            }],
        };

        let serialized = serde_json::to_string(&config).unwrap();
        assert_eq!(
            "{\"initial_zoom_level\":1,\"speed_modifier\":4.0,\"zoom_levels\":[{\"order\":1,\"scale\":[1.0,1.0,1.0]}]}",
            &serialized
        );
    }
}
