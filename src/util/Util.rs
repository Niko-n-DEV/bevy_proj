#![allow(unused)] // Удалить потом
use bevy::prelude::*;

// use crate::core::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct IVec2C {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Vec2C {
    pub x: f32,
    pub y: f32,
}

// #[derive(Component)]
// pub struct Follow {
//     pub target: Entity
// }

// impl Follow {
//     fn new(target: Entity) -> Self {
//         Follow { target }
//     }
// }

// pub fn follow_to (
//     mut commands: Commands,
//     query: Query<(Entity, &Transform, &Follow), Without<Transform>>,
// ) {
//     for (entity, target_transform, follow) in query.iter() {
//         commands.entity(entity).insert(target_transform.clone());
//     }
// }

// fn camera_follow_player(
//     player_query: Query<&Transform, With<PlayerEntity>>,
//     mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<PlayerEntity>)>,
// ) {
//     let player_tranform = player_query.single().translation;
//     let mut camera_transform = camera_query.single_mut();

//     camera_transform.translation.x = player_tranform.x;
//     camera_transform.translation.y = player_tranform.y;
// }
