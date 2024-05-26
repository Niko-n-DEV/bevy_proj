#![allow(unused)]
use bevy::prelude::*;

use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize
};

use crate::core::{
    ItemType::{
        ItemType,
        ItemSizeType,
        ItemStackType,
    },
    EntityType::EntityType,
    ObjType::ObjectSizeType,
    resource::graphic::Atlas::{
        AtlasType,
        AtlasRes,
    },
    Craft::CraftResult,
    Util::{
        IVec2C,
        Vec2C
    }
};

#[derive(Serialize, Deserialize)]
pub struct ModuleRegistry {
    pub id:     String,
    pub name:   String,
    pub desc:   String
}

#[derive(Resource)]
pub struct Registry {
    pub module_registry: HashMap<String, ModuleRegistry>,
    pub entity_registry: HashMap<String, EntityRegistry>,    // Хэш-таблица с регистрируемыми сущностями
    pub object_registry: HashMap<String, ObjectRegistry>,    // Хэш-таблица с регистрируемыми объектами
    pub item_registry:   HashMap<String, ItemRegistry>,      // Хэш-таблица с регистрируемыми предметами

    pub test:            HashMap<String, TestRegistry>,    // Хэш-таблица с тест
}

#[derive(Serialize, Deserialize)]
pub struct EntityRegistry {
    pub id_name:        String,
    pub id_source:      Option<String>,
    pub id_texture_b:   String,
    pub id_texture_h:   Option<String>,
    pub entity_type:    EntityType,
    pub health:         f32
}

#[derive(Serialize, Deserialize)]
pub struct ObjectRegistry {
    pub id_name:        String,
    pub id_source:      Option<String>,
    pub id_texture:     String,
    pub size:           IVec2C,
    pub collision:      Vec2C,
    pub durability:     Option<usize>
}

// pub struct EntityObjectDefinition {
//     pub id_name: String,
//     pub components: HashMap<String, Component>,
// }

impl ObjectRegistry {
    /// Получение прочности предмета
    pub fn get_base_durability(&self) -> Option<usize> {
        self.durability
    }
}

#[derive(Serialize, Deserialize)]
pub struct ItemRegistry {
    pub id_name:    String,
    pub id_source:  Option<String>,
    pub id_texture: String,
    pub item_type:  ItemType,
    pub item_size:  ItemSizeType,
    pub stackable:  Option<ItemStackType>,
    pub stack_size: Option<usize>,
    pub durability: Option<usize>
}

impl ItemRegistry {
    /// Получение прочности предмета
    pub fn get_base_durability(&self) -> Option<usize> {
        self.durability
    }

    // Получение размера стака
    pub fn get_stack_size(&self) -> Option<usize> {
        self.stack_size
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecipeRegistry {
    pub request:    Vec<String>,
    pub result:     CraftResult,
}

pub struct TestRegistry(pub String);

impl Registry {
    pub fn new() -> Self {
        Self {
            module_registry: HashMap::new(),

            entity_registry: HashMap::new(),
            object_registry: HashMap::new(),
            item_registry:   HashMap::new(),

            test:            HashMap::new()
        }
    }

    // ==============================
    // Module - Founder
    // ==============================
    pub fn register_module(&mut self, module_type: ModuleRegistry) {
        if !self.module_registry.contains_key(&module_type.id) {
            self.module_registry.insert(module_type.id.clone(), module_type);
        }
    }

    // ==============================
    // Entity
    // ==============================
    pub fn register_entity(&mut self, entity_type: EntityRegistry) {
        if !self.entity_registry.contains_key(&entity_type.id_name) {
            println!("Register Entity: {}", &entity_type.id_name);
            self.entity_registry.insert(entity_type.id_name.clone(), entity_type);
        }
    }

    pub fn get_entity_texture(&self, name: &str, atlas: &AtlasRes) -> Option<SpriteSheetBundle> {
        atlas.get_spritesheet(AtlasType::Entity, name) // -> graphic/Atlas
    }

    pub fn get_entity_info(&self, name: &str) -> Option<&EntityRegistry> {
        self.entity_registry.get(name)
    }

    // ==============================
    // Objects
    // ==============================
    pub fn register_object(&mut self, object_type: ObjectRegistry) {
        if !self.object_registry.contains_key(&object_type.id_name) {
            println!("Register Object: {}", &object_type.id_name);
            self.object_registry.insert(object_type.id_name.clone(), object_type);
        }
    }

    pub fn get_object_texture(&self, name: &str, atlas: &AtlasRes) -> Option<SpriteSheetBundle> {
        atlas.get_spritesheet(AtlasType::Objects, name) // -> graphic/Atlas
    }

    pub fn get_object_info(&self, name: &str) -> Option<&ObjectRegistry> {
        self.object_registry.get(name)
    }

    // ==============================
    // Items
    // ==============================
    pub fn register_item(&mut self, item_type: ItemRegistry) {
        if !self.item_registry.contains_key(&item_type.id_name) {
            println!("Register Item: {}", &item_type.id_name);
            self.item_registry.insert(item_type.id_name.clone(), item_type);
        }
    }

    pub fn get_item_texture(&self, name: &str, atlas: &AtlasRes) -> Option<SpriteSheetBundle> {
        atlas.get_spritesheet(AtlasType::Items, name) // -> graphic/Atlas
    }

    pub fn get_item_info(&self, name: &str) -> Option<&ItemRegistry> {
        self.item_registry.get(name)
    }

    // ==============================
    // Test
    // ==============================
    pub fn register_test(&mut self, id: String, test_type: TestRegistry) {
        if !self.test.contains_key(&id) {
            self.test.insert(id, test_type);
        }
    }

    pub fn get_test(&self, name: &str, atlas: &AtlasRes) -> Option<SpriteSheetBundle> {
        if let Some(var) = self.test.get(name) {
            atlas.get_spritesheet(AtlasType::Test, &var.0)
        } else {
            None
        }
    }
}