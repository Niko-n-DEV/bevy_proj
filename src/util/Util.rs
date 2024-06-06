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

#[derive(Component)]
pub struct AttachTo {
    pub offset: Vec2,
}

#[derive(Component)]
pub struct ToAttach;

pub fn attach_objects(
    mut attachto_query:  Query<(&mut Transform, &AttachTo), With<AttachTo>>,
        toattach_query: Query<&mut Transform, With<ToAttach>>
) {
    for transform in toattach_query.iter() {
        for mut transform_mut in attachto_query.iter_mut() {
            transform_mut.0.translation =
                transform.translation + Vec3::new(transform_mut.1.offset.x, transform_mut.1.offset.y, 0.5);
        }
    }
}

fn create_boundary(start: Vec2, end: Vec2) -> Vec2 {
    Vec2::new((start.x - end.x).abs(), (start.y - end.y).abs())
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
