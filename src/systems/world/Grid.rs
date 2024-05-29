#![allow(unused)]
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};

use std::{
    collections::HashSet, 
    marker::PhantomData, 
    ops::Index
};

use futures_lite::future;

pub const GRID_SIZE: usize = 200;

//
//
//

pub struct GridSize {
    pub grid_size: i32
}

impl GridSize {
    pub fn get_grid_size(&self) -> i32 {
        self.grid_size
    }
}

#[derive(Resource)]
pub struct Grid<T> {
    pub entities: [[Option<Entity>; 16]; 16],
    _marker: PhantomData<T>,
}

//
//
//

#[derive(Resource)]
pub struct ConnectedComponents<T> {
    pub components: Vec<HashSet<GridLocation>>,
    _marker: PhantomData<T>,
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            entities: [[None; 16]; 16],
            _marker: Default::default(),
        }
    }
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug, Deref, DerefMut)]
pub struct GridLocation(pub IVec2);

impl GridLocation {
    pub fn new(x: i32, y: i32) -> Self {
        GridLocation(IVec2::new(x as i32, y as i32))
    }

    pub fn from_world(position: Vec2) -> Option<Self> {
        let position = position + Vec2::splat(0.5);
        let location = GridLocation(IVec2::new(position.x as i32, position.y as i32));

        if Grid::<()>::valid_index(&location) {
            Some(location)
        } else {
            None
        }
    }
}

impl From<IVec2> for GridLocation {
    fn from(value: IVec2) -> Self {
        GridLocation(value)
    }
}

impl<T> Grid<T> {
    pub fn occupied(&self, location: &GridLocation) -> bool {
        Grid::<T>::valid_index(location) && self[location].is_some()
    }

    pub fn valid_index(location: &GridLocation) -> bool {
        location.x >= 0
            && location.y >= 0
            && location.x < GRID_SIZE as i32
            && location.y < GRID_SIZE as i32
    }
}

impl<T> Index<&GridLocation> for Grid<T> {
    type Output = Option<Entity>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.entities[index.x as usize][index.y as usize]
    }
}

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

fn update_connected_components<T: Component>(
    mut commands: Commands,
    grid: Res<Grid<T>>,
    mut events: EventReader<DirtyGridEvent<T>>,
    // Should maybe be a resource?
    current_tasks: Query<Entity, With<ConnectedTask<T>>>,
) {
    if !events.is_empty() {
        events.clear();
        for task in &current_tasks {
            commands.entity(task).despawn_recursive();
        }

        let thread_pool = AsyncComputeTaskPool::get();
        let grid = Box::new(grid.clone());

        let task = thread_pool.spawn(async move {
            let starts = all_points(100) // test
                .into_iter()
                .filter(|point| !grid.occupied(point))
                .collect::<Vec<_>>();

            // ConnectedComponents::<T> {
            //     components: connected_components::connected_components(&starts, |p| {
            //         neumann_neighbors(&grid, p)
            //     }),
            //     ..default()
            // }
        });

        // commands.spawn(ConnectedTask { task });
    }
}

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

fn all_points(size: i32) -> Vec<GridLocation> {
    (0..size)
        .flat_map(|x| (0..size).map(move |y| GridLocation::new(x as i32, y as i32)))
        .collect()
}

impl<T> Default for ConnectedComponents<T> {
    fn default() -> Self {
        Self {
            components: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<T> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            entities: self.entities,
            _marker: self._marker,
        }
    }
}