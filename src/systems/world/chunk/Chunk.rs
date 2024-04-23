use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Component)]
pub struct Chunk {
    pub chunk_pos: IVec2,
    pub objects: HashMap<IVec2, Entity>
}