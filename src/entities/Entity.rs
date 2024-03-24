#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    player::PlayerEntity::PlayerEntity,
    Movement::DirectionState
};

/// Компонент отвечающий за [Здоровье]
#[derive(Component)]
pub struct Health(pub f32);

/// Компонент отвечающий за [Скорость]
#[derive(Component)]
pub struct Speed(f32);

/// Базовый компонент отвечающий за основу [Entity]
#[derive(Component)]
pub struct EntityBase {
    pub health: Health,
    pub direction: DirectionState,
    pub speed: f32,
}

#[derive(Component, Default)]
pub enum EntityState {
    #[default]
    Idle,
    Move,
}

#[derive(Event)]
pub struct EntityCollisionEvent;

// ==================================================
// Test
// ==================================================
pub fn update_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&EntityBase, &mut Transform, Entity), Without<PlayerEntity>>,
    player_query: Query<(&PlayerEntity, &mut Transform), Without<EntityBase>>,
    mut commands: Commands,
) {
    if let Ok((_player_movement, player_transform)) = player_query.get_single() {
        for (enemy, mut transform, entity) in enemy_query.iter_mut() {
            let moving = Vec3::normalize(player_transform.translation - transform.translation)
                * enemy.speed
                * time.delta_seconds();
            transform.translation += moving;

            if enemy.health.0 <= 0. {
                commands.entity(entity).despawn();
            }
        }
    }
}
