use bevy::ecs::system::Resource;

use serde::{
    Deserialize,
    Serialize
};

use std::fs;

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub test1: String,
    pub test2: i32,
    pub vsync: bool,
    pub rendering_distance: i32,
    pub chunk_size: i32,
}

impl Settings {
    pub fn load() -> Settings {
        if let Ok(contents) = fs::read_to_string("G:\\settings.json") {
            if let Ok(settings) = serde_json::from_str(&contents) {
                return settings;
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            fs::write("G:\\settings.json", json).ok();
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            test1: "default_value".to_string(),
            test2: 42,
            vsync: false,
            rendering_distance: 3,
            chunk_size: 16,
        }
    }
}
