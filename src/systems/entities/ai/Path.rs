#![allow(unused)]
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};

use pathfinding::prelude::astar;
use futures_lite::future;

use std::collections::VecDeque;
use std::sync::Arc;

use crate::core::world::Grid::{Grid, GridLocation};

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
    pub locations: VecDeque<(IVec2, UVec2)>,
}

pub struct Path {
    pub steps: Vec<GridLocation>,
}

impl Path {
    pub fn optimize_corners(&mut self) {
        // i должно отслеживаться здесь, т.к. вектор меняется
        let mut i = 0;
        while i + 2 < self.steps.len() {
            let first_step = &self.steps[i];
            let third_step = &self.steps[i + 2];
            // Если и x, и y изменяются, то это угол
            if first_step.0.x != third_step.0.x && first_step.0.y != third_step.0.y {
                self.steps.remove(i + 1);
            }
            i += 1;
        }
    }
}

pub fn neumann_neighbors(grid: &Grid, location: &GridLocation) -> Vec<GridLocation> {
    let (x, y) = (location.0.x as i32, location.0.y as i32);

    // Вектор с подтверждённым путём без препятствий
    let mut sucessors = Vec::new();

    // Проверка пути слева от точки вычисления
    if let Some(left) = x.checked_sub(1) {
        let location = GridLocation::new(left, y);
        if !grid.check_exist_object(&location.0) {
            sucessors.push(location);
        }
    }
    // Проверка пути снизу от точки вычисления
    if let Some(down) = y.checked_sub(1) {
        let location = GridLocation::new(x, down);
        if !grid.check_exist_object(&location.0) {
            sucessors.push(location);
        }
    }
    // Проверка пути справа от точки вычисления
    if let Some(right) = x.checked_add(1) {
        let location = GridLocation::new(right, y);
        if !grid.check_exist_object(&location.0) {
            sucessors.push(location);
        }
    }
    // Проверка пути сверху от точки вычисления
    if let Some(up) = y.checked_add(1) {
        let location = GridLocation::new(x, up);
        if !grid.check_exist_object(&location.0) {
            sucessors.push(location);
        }
    }
    sucessors
}

impl Grid {
    pub fn path_to(
        &self,
        start:  &GridLocation,
        goal:   &GridLocation,
    ) -> Result<Path, PathfindingError> {
        let result = astar(
            start,
            |p| {
                neumann_neighbors(self, p)
                    .iter()
                    .map(|neighbor| (neighbor.clone(), 1))
                    .collect::<Vec<_>>()
            },
            |p| p.distance(goal) / 3,
            |p| p == goal,
        );

        if let Some((steps, _length)) = result {
            Ok(Path { steps })
        } else {
            Err(PathfindingError)
        }
    }
}

pub fn calculate_pos_V2(target: (IVec2, UVec2)) -> Vec2 {
    Vec2::new(
        ((target.0.x as f32 * 256.0) + (target.1.x as f32 * 16.0) + 8.0), 
        ((target.0.y as f32 * 256.0) + (target.1.y as f32 * 16.0) + 8.0)
    )
}

pub fn calculate_pos_IV2(target: (IVec2, IVec2)) -> IVec2 {
    IVec2::new(
        ((target.0.x * 256) + (target.1.x * 16) + 8), 
        ((target.0.y * 256) + (target.1.y * 16) + 8)
    )
}

#[derive(Component)]
pub struct PathfindingTask(Task<Result<Path, PathfindingError>>);

pub fn spawn_optimized_pathfinding_task<T: Component>(
    commands:   &mut Commands,
    target:     Entity,
    grid:       Grid,
    start:      GridLocation,
    end:        GridLocation,
) {
    // Выход, если в конец нельзя прийти
    if grid.check_exist_object(&end.0) {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();

    // Must clone because the grid can change between frames
    // Must box to prevent stack overflows on very large grids
    let grid_arc = Arc::new(grid);

    let task = thread_pool.spawn(async move {
        let mut path = grid_arc.path_to(&start, &end);
        let _ = path.as_mut().map(|p| p.optimize_corners());
        path
    });

    commands.entity(target).insert(PathfindingTask(task));
}

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
                            .push_back(location.get_chunk_and_local());
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