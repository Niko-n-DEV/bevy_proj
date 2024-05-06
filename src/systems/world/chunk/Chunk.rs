use std::collections::HashMap;

use bevy::prelude::*;

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