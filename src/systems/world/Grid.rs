#![allow(unused)]
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};

use std::{
    collections::{
        HashMap,
        HashSet,
    }, 
    marker::PhantomData, 
    ops::Index
};

use crate::core::{
    world::chunk::Chunk::ChunkX as Chunk,
    resource::graphic::Atlas::{
        AtlasRes,
        AtlasType,
    }
};

use futures_lite::future;

pub const CHUNK_SIZE: i32 = 256;

//
//
//

#[derive(Resource)]
pub struct Grid {
    pub chunks:             HashMap<IVec2, Chunk>,
    pub render_distance:    i32,
    pub debug_mode:         bool,
    pub debug_chunks:       HashMap<IVec2, Entity>,
}

impl Grid {
    pub fn new(render_distance: i32) -> Self {
        Self {
            chunks:         HashMap::new(),
            render_distance,
            debug_mode:     !false,
            debug_chunks:   HashMap::new()
        }
    }

    pub fn load_chunk(
        &mut self,
        cmd:       &mut Commands,
        atlas:     &AtlasRes,
        chunk_pos: IVec2
    ) {

        if !self.chunks.contains_key(&chunk_pos) {
            self.chunks.insert(chunk_pos, Chunk::new(chunk_pos));
            // println!("Chunk at position {:?} loaded.", chunk_pos);
        }

        if self.debug_mode {
            if !self.debug_chunks.contains_key(&chunk_pos) {
                if let Some(img) = atlas.get_texture(AtlasType::Ui, "debug_chunk") {
                    let chunk = cmd
                        .spawn(SpriteSheetBundle {
                            sprite: Sprite {
                                anchor: bevy::sprite::Anchor::BottomLeft,
                                ..default()
                            },
                            texture: img.1,
                            atlas: img.0,
                            transform: Transform {
                                translation: Vec3::new(chunk_pos.x as f32 * 256.0, chunk_pos.y as f32 * 256.0, -1.0),
                                scale: Vec3::new(16.0, 16.0, 0.0),
                                ..default()
                            },
                            ..default()
                        })
                    .insert(Name::new(format!("{chunk_pos}_debug_chunk")))
                    .id();
                    self.debug_chunks.insert(chunk_pos, chunk);
                } else {
                    warn!("Texture \"debug_chunk\" is not found!");
                }
            }
        }
    }

    pub fn unload_chunk(
        &mut self, 
        cmd:       &mut Commands,
        atlas:     &AtlasRes,
        chunk_pos: &IVec2
    ) {
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.chunk_remove(cmd);
            
            self.chunks.remove(&chunk_pos);
        }

        if self.debug_mode {
            if let Some(entity) = self.debug_chunks.remove(&chunk_pos) {
                cmd.entity(entity).despawn();
            }
        }
    }

    pub fn update_chunks(
        &mut self,
        cmd:        &mut Commands,
        atlas:      &AtlasRes,
        player_pos: IVec2
    ) {
        let current_chunk = get_format_current_chunk(player_pos);
        let half_render_distance = self.render_distance / 2;

        let mut chunks_to_load = Vec::new();
        let mut chunks_to_unload = Vec::new();

        for x in -half_render_distance..=half_render_distance {
            for y in -half_render_distance..=half_render_distance {
                let chunk_pos = IVec2::new(current_chunk.x + x, current_chunk.y + y);
                if !self.chunks.contains_key(&chunk_pos) {
                    chunks_to_load.push(chunk_pos);
                }
            }
        }

        for chunk_pos in self.chunks.keys() {
            if (chunk_pos.x - current_chunk.x).abs() > half_render_distance || (chunk_pos.y - current_chunk.y).abs() > half_render_distance {
                chunks_to_unload.push(*chunk_pos);
            }
        }

        for chunk_pos in chunks_to_load {
            self.load_chunk(cmd, atlas, chunk_pos);
        }

        for chunk_pos in chunks_to_unload {
            self.unload_chunk(cmd, atlas, &chunk_pos);
        }
    }

    pub fn add_object_to_chunk(&mut self, entity: Entity, coord: IVec2) -> bool {
        if let Some(chunk) = self.chunks.get_mut(&get_format_current_chunk(coord)) {
            chunk.add_object(entity, coord)
        } else {
            false
        }
    }

    pub fn add_subject_to_chunk(&mut self, chunk_pos: IVec2, entity: Entity, coord: IVec2) -> bool {
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.add_subject(entity, coord)
        } else {
            false
        }
    }

    /// По идеи проверка есть на том положении объект
    pub fn is_object_present(&self, coord: IVec2) -> bool {
        if let Some(chunk) = self.chunks.get(&coord) {
            chunk.is_object_present(coord)
        } else {
            false
        }
    }

    /// Проверка наличия объекта по данным глобальным координатам
    pub fn check_exist_object(&self, coord: &IVec2) -> bool {
        if let Some(chunk) = self.chunks.get(&get_format_current_chunk(*coord)) {
            chunk.check_object(*coord)
        } else {
            false
        }
    } 

    pub fn check_exist_object_ex(&self, chunk: IVec2, local: IVec2) {
        if let Some(chunk) = self.chunks.get(&chunk) {

        }
    }

    pub fn upload_all(&mut self, commands: &mut Commands) {
        let keys: Vec<IVec2> = self.chunks.keys().cloned().collect();

        for coord in keys {
            if let Some(chunk) = self.chunks.get_mut(&coord) {
                chunk.chunk_remove(commands);
            }
        }
        
        self.chunks.clear();

        if !self.debug_chunks.is_empty() {
            let keys: Vec<IVec2> = self.debug_chunks.keys().cloned().collect();

            for coord in keys {
                if let Some(chunk) = self.debug_chunks.get_mut(&coord) {
                    commands.entity(*chunk).despawn_recursive();
                }
            }
            
            self.debug_chunks.clear();
        }
    }
}

/// Получение координат чанка по вводным данным
pub fn get_format_current_chunk(input_var: IVec2) -> IVec2 {
    let mut chunk_x = input_var.x / CHUNK_SIZE;
    let mut chunk_y = input_var.y / CHUNK_SIZE;
    if input_var.x < 0 {
        chunk_x -= 1;
    }
    if input_var.y < 0 {
        chunk_y -= 1;
    }
    IVec2::new(chunk_x, chunk_y)
}

pub fn global_to_local(coord: IVec2) -> UVec2 {
    UVec2::new(
        (coord.x.abs() as u32 % 256) / 16,
        (coord.y.abs() as u32 % 256) / 16
    )
}

//
//
//

#[derive(Resource)]
pub struct ConnectedComponents<T> {
    pub components: Vec<HashSet<GridLocation>>,
    _marker: PhantomData<T>,
}

// impl<T> Default for Grid<T> {
//     fn default() -> Self {
//         Self {
//             entities: [[None; 16]; 16],
//             _marker: Default::default(),
//         }
//     }
// }

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GridLocation(pub IVec2);

impl GridLocation {
    pub fn new(x: i32, y: i32) -> Self {
        GridLocation(IVec2::new(x, y))
    }

    pub fn get_chunk_and_local(&self) -> (IVec2, UVec2) {
        (
            get_format_current_chunk(self.0),
            global_to_local(self.0)
        )
    }

    pub fn distance(&self, other: &GridLocation) -> usize {
        (self.0.x.abs_diff(other.0.x) + self.0.y.abs_diff(other.0.y)) as usize
    }

    // pub fn from_world(position: Vec2) -> Option<Self> {
    //     let position = position + Vec2::splat(0.5);
    //     let location = GridLocation(IVec2::new(position.x as i32, position.y as i32));

    //     if Grid::valid_index(&location) {
    //         Some(location)
    //     } else {
    //         None
    //     }
    // }
}

// impl From<(IVec2, IVec2)> for GridLocation {
//     fn from(chunk: IVec2, local: IVec2) -> Self {
//         GridLocation(chunk, local)
//     }
// }

// impl Index<&GridLocation> for Grid {
//     type Output = Option<Entity>;

//     fn index(&self, index: &GridLocation) -> &Self::Output {
//         &self.entities[index.x as usize][index.y as usize]
//     }
// }

#[derive(Component)]
pub struct LockToGrid;

#[derive(Event)]
pub struct DirtyGridEvent<T>(pub GridLocation, PhantomData<T>);

#[derive(Default)]
pub struct GridPlugin<T> {
    _marker: PhantomData<T>,
}

#[derive(Component)]
struct ConnectedTask<T> {
    task: Task<ConnectedComponents<T>>,
}

fn resolve_connected_components<T: Component>(
    mut commands: Commands,
    mut connected: ResMut<ConnectedComponents<T>>,
    // Should maybe be a resource?
    mut tasks: Query<(Entity, &mut ConnectedTask<T>)>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.task)) {
            //TODO is there a way to make bevy auto remove these or not panic or something
            commands.entity(task_entity).despawn_recursive();
            *connected = result;
        }
    }
}

// fn update_connected_components<T: Component>(
//     mut commands: Commands,
//     grid: Res<Grid>,
//     mut events: EventReader<DirtyGridEvent<T>>,
//     // Should maybe be a resource?
//     current_tasks: Query<Entity, With<ConnectedTask<T>>>,
// ) {
//     if !events.is_empty() {
//         events.clear();
//         for task in &current_tasks {
//             commands.entity(task).despawn_recursive();
//         }

//         let thread_pool = AsyncComputeTaskPool::get();
//         let grid = Box::new(grid.clone());

//         let task = thread_pool.spawn(async move {
//             let starts = all_points(100) // test
//                 .into_iter()
//                 .filter(|point| !grid.occupied(point))
//                 .collect::<Vec<_>>();

//             // ConnectedComponents::<T> {
//             //     components: connected_components::connected_components(&starts, |p| {
//             //         neumann_neighbors(&grid, p)
//             //     }),
//             //     ..default()
//             // }
//         });

//         // commands.spawn(ConnectedTask { task });
//     }
// }

// fn remove_from_grid<T: Component>(
//     mut grid: ResMut<Grid<T>>,
//     mut query: RemovedComponents<T>,
//     mut dirty: EventWriter<DirtyGridEvent<T>>,
// ) {
//     for removed_entity in query.iter() {
//         // Search for entity
//         let removed = grid.iter().find(|(entity, _)| *entity == removed_entity);
//         if let Some((_, location)) = removed {
//             dirty.send(DirtyGridEvent::<T>(
//                 location.clone(),
//                 PhantomData::default(),
//             ));
//             grid[&location] = None;
//         }
//     }
// }

// fn add_to_grid<T: Component>(
//     mut grid: ResMut<Grid<T>>,
//     query: Query<(Entity, &GridLocation), Added<T>>,
//     mut dirty: EventWriter<DirtyGridEvent<T>>,
// ) {
//     for (entity, location) in &query {
//         if let Some(existing) = grid[location] {
//             if existing != entity {
//                 warn!("Over-writing entity in grid");
//                 dirty.send(DirtyGridEvent::<T>(
//                     location.clone(),
//                     PhantomData::default(),
//                 ));
//                 grid[location] = Some(entity);
//             }
//         } else {
//             dirty.send(DirtyGridEvent::<T>(
//                 location.clone(),
//                 PhantomData::default(),
//             ));
//             grid[location] = Some(entity);
//         }
//     }
// }

// fn all_points(size: i32) -> Vec<GridLocation> {
//     (0..size)
//         .flat_map(|x| (0..size).map(move |y| GridLocation::new(x as i32, y as i32)))
//         .collect()
// }

impl<T> Default for ConnectedComponents<T> {
    fn default() -> Self {
        Self {
            components: Default::default(),
            _marker: Default::default(),
        }
    }
}

// impl Clone for Grid {
//     fn clone(&self) -> Self {
//         Self {
//             entities: self.entities,
//         }
//     }
// }