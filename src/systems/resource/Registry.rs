#![allow(unused)]
use bevy::prelude::*;

use std::collections::HashMap;

use crate::core::Entity::EntityType;

//pub type EntityRegistry = HashMap<EntityType, Vec<String>>;

#[derive(Resource)]
pub struct Registry {
    pub entity_registry: HashMap<String, EntityType>    // Хэш-таблица с регистрируемыми сущностями
    // Хэш-таблица с 
}

impl Registry {
    pub fn new() -> Self {
        Self {
            entity_registry: HashMap::new()
        }
    }

    pub fn register_entity_type(&mut self, id: String, entity_type: EntityType) {
        if !self.entity_registry.contains_key(&id) {
            self.entity_registry.insert(id, entity_type);
        }
    }
}

// fn register_type(type_registry: &mut EntityRegistry, type_for_mounting: EntityType, new_name: String) {
    
// }