#![allow(unused)]
use crate::core::*;

use serde::{Deserialize, Serialize};
use serde_json::*;

use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub test1: String,
    pub test2: i32,
    pub rendering_distance: i32,
    pub chunk_size: i32,
}

impl Settings {
    // Функция для чтения настроек из файла
    pub fn load() -> Settings {
        if let Ok(contents) = fs::read_to_string("G:\\settings.json") {
            // Если файл существует, десериализуем его содержимое
            if let Ok(settings) = serde_json::from_str(&contents) {
                return settings;
            }
        }

        // Если чтение не удалось или файл не существует, возвращаем значения по умолчанию
        Self::default()
    }

    // Функция для сохранения настроек в файл
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            // Сохраняем в файл
            fs::write("G:\\settings.json", json).ok();
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        // Значения по умолчанию
        Settings {
            test1: "default_value".to_string(),
            test2: 42,
            rendering_distance: 3,
            chunk_size: 16, // Установите значения по умолчанию для других полей
        }
    }
}
