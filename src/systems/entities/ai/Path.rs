#![allow(unused)]
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

use std::collections::VecDeque;

use crate::core::world::Grid::GridLocation;

//
//
//

pub fn path_finding_plugin(app: &mut App) {
    // app.add_systems(Update, apply_pathfinding_to_ai);
}

//
//
//

#[derive(Component, Default)]
pub struct AiPath {
    pub locations: VecDeque<(Vec2, Vec2)>,
}

pub struct Path {
    pub steps: Vec<GridLocation>,
}

// impl Path {
//     pub fn optimize_corners(&mut self) {
//         // i must be tracked here because vec len changes
//         let mut i = 0;
//         while i + 2 < self.steps.len() {
//             let first_step = &self.steps[i];
//             let third_step = &self.steps[i + 2];
//             //If both x and y change then this is a corner
//             if first_step.x != third_step.x && first_step.y != third_step.y {
//                 self.steps.remove(i + 1);
//             }
//             i += 1;
//         }
//     }
// }

#[derive(Component)]
pub struct PathfindingTask(Task<Result<Path, PathfindingError>>);

// pub fn spawn_optimized_pathfinding_task<T: Component>(
//     commands: &mut Commands,
//     target: Entity,
//     grid: &Grid<T>,
//     start: GridLocation,
//     end: GridLocation,
// ) {
//     // Fail early if end is not valid
//     if grid.occupied(&end) {
//         return;
//     }

//     let thread_pool = AsyncComputeTaskPool::get();

//     // Must clone because the grid can change between frames
//     // Must box to prevent stack overflows on very large grids
//     let grid = Box::new(grid.clone());

//     let task = thread_pool.spawn(async move {
//         let mut path = grid.path_to(&start, &end);
//         let _ = path.as_mut().map(|p| p.optimize_corners());
//         path
//     });

//     commands.entity(target).insert(PathfindingTask(task));
// }

pub fn apply_pathfinding_to_ai(
    mut commands:   Commands,
    mut paths:      Query<&mut AiPath>,
    mut tasks:      Query<(Entity, &mut PathfindingTask)>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(task_entity).remove::<PathfindingTask>();

            if let Ok(mut ai_path) = paths.get_mut(task_entity) {
                if let Ok(path) = result {
                    ai_path.locations.clear();
                    for location in path.steps.iter() {
                        ai_path
                            .locations
                            .push_back((location.0.as_vec2(), location.1.as_vec2()));
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PathfindingError;

// #[cfg(test)]
// mod tests {

//     use bevy::prelude::Entity;

//     use crate::core::world::Grid::{Grid, GridLocation};

//     #[test]
//     fn basic_pathfinding() {
//         let goal = GridLocation::new(4, 6);
//         let start = GridLocation::new(1, 1);
//         let mut grid: Grid = Grid::default();
//         grid.entities[2][0] = Some(Entity::from_raw(0));
//         grid.entities[2][1] = Some(Entity::from_raw(0));
//         grid.entities[2][2] = Some(Entity::from_raw(0));

//         let result = grid.path_to(&start, &goal);
//         assert!(result.is_ok());
//     }
// }