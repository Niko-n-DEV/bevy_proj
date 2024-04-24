use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Component, Resource)]
pub struct Chunk {
    pub chunk_pos: IVec2,
    pub objects: HashMap<IVec2, Entity>
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            chunk_pos: IVec2::ZERO,
            objects: HashMap::new()
        }
    }
}