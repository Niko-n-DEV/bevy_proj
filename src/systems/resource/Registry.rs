#![allow(unused)]
use bevy::prelude::*;

use std::collections::HashMap;

use crate::core::{
    ItemType::ItemType,
    Entity::EntityType,
    resource::graphic::Atlas::AtlasRes
};

#[derive(Resource)]
pub struct Registry {
    pub entity_registry: HashMap<String, EntityRegistry>,    // Хэш-таблица с регистрируемыми сущностями
    pub object_registry: HashMap<String, ObjectRegistry>,    // Хэш-таблица с регистрируемыми объектами
    pub item_registry:   HashMap<String, ItemRegistry>,      // Хэш-таблица с регистрируемыми предметами

    pub test:            HashMap<String, TestRegistry>,    // Хэш-таблица с регистрируемыми тест
}

pub struct EntityRegistry {
    pub id_name:        String,
    pub entity_type:    EntityType,
    pub id_texture:     usize
}

pub struct ObjectRegistry {
    pub id_name: String,
}

pub struct ItemRegistry {
    pub id_name: String,
    pub item_type: ItemType
}

pub struct TestRegistry(pub String);

impl Registry {
    pub fn new() -> Self {
        Self {
            entity_registry: HashMap::new(),
            object_registry: HashMap::new(),
            item_registry:   HashMap::new(),

            test:            HashMap::new()
        }
    }

    // ==============================
    // Entity
    // ==============================
    pub fn register_entity(&mut self, id: String, entity_type: EntityRegistry) {
        if !self.entity_registry.contains_key(&id) {
            self.entity_registry.insert(id, entity_type);
        }
    }

    pub fn get_entity(&mut self, name: &str, atlas: &AtlasRes) -> Option<SpriteSheetBundle> {
        if let Some(var) = self.entity_registry.get(name) {
            atlas.get_entity_spritesheet(&var.id_name)
        } else {
            None
        }
    }

    // ==============================
    // Objects
    // ==============================

    // ==============================
    // Items
    // ==============================
    pub fn register_item(&mut self, id: String, item_type: ItemRegistry) {
        if !self.item_registry.contains_key(&id) {
            self.item_registry.insert(id, item_type);
        }
    }



    // ==============================
    // Test
    // ==============================
    pub fn register_test(&mut self, id: String, test_type: TestRegistry) {
        if !self.test.contains_key(&id) {
            self.test.insert(id, test_type);
        }
    }

    pub fn get_test(&mut self, name: &str, atlas: &AtlasRes) -> Option<SpriteSheetBundle> {
        if let Some(var) = self.test.get(name) {
            atlas.get_test_spritesheet(&var.0)
        } else {
            None
        }
    }
}