#![allow(unused)]
#![allow(non_snake_case)]
pub mod Path;

use std::sync::Arc;

use bevy::prelude::*;

use crate::core::{
    Entity::EntityBase,
    entities::{
        EntitySystem::MovementEntity,
        ai::Path::{
            path_finding_plugin,
            AiPath,
            PathfindingTask,
            calculate_pos_V2,
            spawn_optimized_pathfinding_task
        },
    },
    Needs::{
        Hunger,
        Recreation
    },
    world::Grid::{
        Grid,
        GridLocation
    },
    UserSystem::UserControl,
    AppState
};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(path_finding_plugin);
        app.add_systems(Update, 
            (
                to_player,
                follow_path.after(to_player)
            ).run_if(in_state(AppState::Game))
        );
    }
}

#[derive(Component)]
pub struct Pawn;

#[derive(Component, Default)]
pub struct Brain {
    state: BrainState,
}

pub enum BrainState {
    Wander(f32),
    GetFood,
    OperateMachine(Entity),
    Relax,
}

impl Default for BrainState {
    fn default() -> Self {
        BrainState::Wander(0.0)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct LastDirection(pub Vec2);

fn update_brains(mut brains: Query<(&mut Brain, &Hunger, &Recreation)>) {
    for (mut brain, hunger, _recreation) in &mut brains {

        if matches!(brain.state, BrainState::OperateMachine(_)) {
            continue;
        }

        if hunger.value < 40.0 {
            brain.state = BrainState::GetFood;
            continue;
        }
        /*
        if recreation.value < 0.4 {
            brain.state = BrainState::Relax;
            sprite.color = Color::BLUE;
            continue;
        }
        */

        if !matches!(brain.state, BrainState::Wander(_)) {
            brain.state = BrainState::Wander(0.0);
        }
    }
}

// fn operate_food_machine(
//     mut brains: Query<(&mut Brain, &mut Hunger), Without<PathfindingTask>>,
//     foods: Query<&FoodMachine>,
//     time: Res<Time>,
// ) {
//     for (mut brain, mut hunger) in &mut brains {
//         let machine = match &brain.state {
//             BrainState::OperateMachine(val) => val,
//             _ => continue,
//         };

//         let food = match foods.get(*machine) {
//             Ok(food) => food,
//             Err(_) => {
//                 warn!("No machine for me to operate :(");
//                 brain.state = BrainState::default();
//                 continue;
//             }
//         };

//         hunger.value += food.rate * time.delta_seconds();
//         if hunger.value >= 100.0 {
//             brain.state = BrainState::default();
//         }
//     }
// }

fn to_player(
    mut commands:   Commands,
    mut brains:     Query<(Entity, &AiPath, &Transform), (Without<PathfindingTask>, Without<UserControl>)>,
        player:     Query<(Entity, &Transform), With<UserControl>>,
        grid:       Res<Grid>,
) {
    if player.is_empty() {
        return;
    }

    for (target, path, transform) in &mut brains {

        // let brain_location = match GridLocation::from_world(transform.translation.truncate()) {
        //     Some(val) => val,
        //     None => {
        //         warn!("AI entity not in grid...");
        //         continue;
        //     }
        // };

        // //FIXME should find closest machine, or better one that can be path found to
        // let (machine_entity, target_point) = match machine_grid
        //     .iter()
        //     .filter(|(ent, _)| food.get(*ent).is_ok())
        //     .map(|(ent, location)| (ent, food.get(ent).unwrap(), location))
        //     .map(|(ent, machine, location)| {
        //         (ent, GridLocation::from(location.0 + machine.use_offset))
        //     })
        //     .filter(|(_ent, location)| components.in_same_component(location, &brain_location))
        //     .min_by_key(|(_, location)| {
        //         FloatOrd(
        //             transform
        //                 .translation
        //                 .truncate()
        //                 .distance((location.0).as_vec2()),
        //         )
        //     }) {
        //     Some(val) => val,
        //     None => {
        //         warn!("No food machines");
        //         continue;
        //     }
        // };

        let player = player.single();

        if path.locations.is_empty() {
            if transform
                .translation
                .truncate()
                .distance(player.1.translation.truncate())
                < 0.5
            {
                println!("Continue path task");
                continue;
            } else {
                println!("Spawn path task");
                spawn_optimized_pathfinding_task(
                    &mut commands,
                    target,
                    Arc::new(grid.clone()),
                    GridLocation::new(transform.translation.x as i32, transform.translation.y as i32),
                    GridLocation::new(player.1.translation.x as i32, player.1.translation.y as i32),
                );
            }
        }
    }
}

// fn clear_path_if_dirty(
//     mut commands: Commands,
//     mut dirty: EventReader<DirtyGridEvent<Wall>>,
//     mut brains: Query<&mut AiPath, Without<PathfindingTask>>,
//     mut pathfinding: Query<Entity, With<PathfindingTask>>,
// ) {
//     for event in dirty.iter() {
//         for mut path in &mut brains {
//             if path
//                 .locations
//                 .iter()
//                 .any(|position| GridLocation::from_world(*position).unwrap() == event.0)
//             {
//                 path.locations.clear();
//             }
//         }
//         for entity in &mut pathfinding {
//             commands.entity(entity).remove::<PathfindingTask>();
//         }
//     }
// }

// fn wander(
//     mut commands: Commands,
//     mut brains: Query<(Entity, &AiPath, &mut Brain, &Transform), Without<PathfindingTask>>,
//     time: Res<Time>,
//     walls: Res<Grid<Wall>>,
//     wall_connected: Res<ConnectedComponents<Wall>>,
// ) {
//     for (target, path, mut brain, transform) in &mut brains {
//         if let BrainState::Wander(last_wander_time) = &mut brain.state {
//             *last_wander_time += time.delta_seconds();
//             if *last_wander_time > 1.0 && path.locations.is_empty() {
//                 *last_wander_time = 0.0;

//                 let mut rng = rand::thread_rng();

//                 if let Some(start) = GridLocation::from_world(transform.translation.truncate()) {
//                     if let Some(end) =
//                         wall_connected.random_point_in_same_component(&start, &mut rng)
//                     {
//                         spawn_optimized_pathfinding_task(
//                             &mut commands,
//                             target,
//                             &walls,
//                             start,
//                             end.clone(),
//                         );
//                     } else {
//                         warn!("I'm in a wall!");
//                     }
//                 } else {
//                     warn!("Entity not in grid");
//                 }
//             }
//         }
//     }
// }

fn follow_path(
    mut paths:  Query<(Entity, &EntityBase, &mut Transform, &mut AiPath, &mut LastDirection)>,
    mut event:  EventWriter<MovementEntity>,
        time:   Res<Time>,
) {
    for (entity, entity_base, mut transform, mut path, mut last_direction) in &mut paths {
        if let Some(next_target) = path.locations.front() {

            let delta = *next_target - transform.translation.truncate();
            let travel_amount = time.delta_seconds();

            if delta.length() > travel_amount * 1.1 {
                let direction = delta.normalize().extend(0.0) * travel_amount;
                last_direction.0 = direction.truncate();
                // transform.translation += direction;
                event.send(MovementEntity(entity, direction, entity_base.speed.0));
            } else {
                path.locations.pop_front();
            }
        } else {
            last_direction.0 = Vec2::ZERO;
        }
    }
}