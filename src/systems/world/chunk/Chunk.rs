#![allow(unused)]
use bevy::prelude::*;

use std::{
    collections::HashMap, 
    marker::PhantomData
};

pub const CHUNK_SIZE: usize = 16;



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

//

pub const GRID_CHUNK_SIZE: usize = 16;

#[derive(Debug, Clone, Component)]
pub struct ChunkEx {
    pub chunk_position: IVec2,
    pub objects:        [[Option<Entity>; GRID_CHUNK_SIZE]; GRID_CHUNK_SIZE],
    pub subjects:       [[Option<Entity>; GRID_CHUNK_SIZE * 2]; GRID_CHUNK_SIZE * 2],
}

impl Default for ChunkEx {
    fn default() -> Self {
        Self {
            chunk_position: IVec2::ZERO,
            objects:        [[None; GRID_CHUNK_SIZE]; GRID_CHUNK_SIZE],
            subjects:       [[None; GRID_CHUNK_SIZE * 2]; GRID_CHUNK_SIZE * 2],
        }
    }
}

impl ChunkEx {
    pub fn new(
        chunk_position: IVec2,
        objects:        Vec<(Entity, IVec2)>,
        subjects:       Vec<(Entity, IVec2)>,
    ) -> Self {
        let mut new_chunk = Self {
            chunk_position,
            objects: [[None; GRID_CHUNK_SIZE]; GRID_CHUNK_SIZE],
            subjects: [[None; GRID_CHUNK_SIZE * 2]; GRID_CHUNK_SIZE * 2],
        };

        for (entity, position) in objects {
            if position.x >= 0 && position.x < GRID_CHUNK_SIZE as i32 && position.y >= 0 && position.y < GRID_CHUNK_SIZE as i32 {
                new_chunk.objects[position.x as usize][position.y as usize] = Some(entity);
            } else {
                println!("Warning: object position {:?} is out of bounds", position);
            }
        }

        for (entity, position) in subjects {
            if position.x >= 0 && position.x < (GRID_CHUNK_SIZE * 2) as i32 && position.y >= 0 && position.y < (GRID_CHUNK_SIZE * 2) as i32 {
                new_chunk.subjects[position.x as usize][position.y as usize] = Some(entity);
            } else {
                println!("Warning: subject position {:?} is out of bounds", position);
            }
        }

        new_chunk
    }

    pub fn check_object(
        &self,
        position: IVec2
    ) -> bool {
        if position.x >= 0 && position.x < GRID_CHUNK_SIZE as i32 && position.y >= 0 && position.y < GRID_CHUNK_SIZE as i32 {
            return self.objects[position.x as usize][position.y as usize].is_some();
        }
        false
    }
}

//
//
//

#[derive(Debug, Clone, Component)]
pub struct ChunkX {
    pub chunk_position: IVec2,
    pub objects:    [[Option<Entity>; 16]; 16],
    pub subjects:   [[Option<Entity>; 32]; 32],
}

impl ChunkX {
    pub fn new(chunk_position: IVec2) -> Self {
        Self {
            chunk_position,
            objects:    [[None; 16]; 16],
            subjects:   [[None; 32]; 32],
        }
    }

    pub fn chunk_remove(&mut self, cmd: &mut Commands) {
        for row in &self.objects {
            for entity_option in row {
                if let Some(entity) = entity_option {
                    cmd.entity(*entity).despawn();
                }
            }
        }
    }

    pub fn add_object(&mut self, entity: Entity, coord: IVec2) -> bool {
        if let Some((x, y)) = self.global_to_local(coord) {
            println!("{} | {}", x, y);
            if self.objects[x][y].is_none() {
                self.objects[x][y] = Some(entity);
                return true;
            }
        }
        false
    }

    pub fn add_subject(&mut self, entity: Entity, coord: IVec2) -> bool {
        let local_coord = self.global_to_local(coord);
        if let Some((x, y)) = local_coord {
            if self.objects[x / 2][y / 2].is_none() && self.subjects[x][y].is_none() {
                self.subjects[x][y] = Some(entity);
                return true;
            }
        }
        false
    }

    pub fn get_object(&mut self, coord: IVec2) -> Option<Entity> {
        if let Some((x, y)) = self.global_to_local(coord) {
            return self.objects[x][y];
        }
        None
    }

    pub fn remove_object(&mut self, object: Entity) -> bool {
        for row in self.objects.iter_mut() {
            if let Some(obj) = row.iter_mut().find(|&&mut find_obj| find_obj == Some(object)) {
                *obj = None;
                return true;
            }
        }
        false
    }

    pub fn is_object_present(&self, coord: IVec2) -> bool {
        if let Some((x, y)) = self.global_to_local(coord) {
            self.objects[x][y].is_some()
        } else {
            false
        }
    }

    fn global_to_local(&self, coord: IVec2) -> Option<(usize, usize)> {
        let local_x = (coord.x.abs() % 256) / 16;
        let local_y = (coord.y.abs() % 256) / 16;
        if local_x >= 0 && local_y >= 0 {
            Some((local_x as usize, local_y as usize))
        } else {
            None
        }
    }
}