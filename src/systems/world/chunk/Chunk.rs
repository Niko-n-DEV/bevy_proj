#![allow(unused)]
use bevy::prelude::*;

use std::collections::HashMap;

#[derive(Component, Resource)]
pub struct Chunk {
    pub chunk_pos: IVec2,
    pub objects: HashMap<IVec2, Entity>,
    pub objects_ex: HashMap<IVec2, Entity>
}

impl Chunk {
    /// Функция для взятия объекта [`Entity`] и удаление записи из HashMap
    pub fn remove_object(&mut self, pos: &IVec2) -> Option<Entity> {
        self.objects.remove(pos)
    }

    pub fn remove_sub_object_ex(&mut self, entity: Entity) {
        let keys: Vec<_> = self.objects_ex.iter()
        .filter(|(_, &ent)| ent == entity)
        .map(|(key, _)| key.clone())
        .collect();

        for key in keys {
            self.objects_ex.remove(&key);
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            chunk_pos: IVec2::ZERO,
            objects: HashMap::new(),
            objects_ex: HashMap::new()
        }
    }
}