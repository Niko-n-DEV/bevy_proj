#![allow(unused)]
use std::default;

use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{player::PlayerEntity::User, Movement::DirectionState};

/// Компонент отвечающий за [Здоровье]
#[derive(Component)]
pub struct Health(pub f32);

/// Компонент отвечающий за [Скорость]
///
/// 0 - Обычная скорость | 1 - Бег | 2 - Медленное передвижение
#[derive(Component)]
pub struct Speed(pub f32, pub f32, pub f32);

/// Компонент отвечающий за [Позицию]
#[derive(Component)]
pub struct Position(pub Vec3);

/// Компонент отвечающий за [Направление движения]
#[derive(Component)]
pub struct Velocity(pub Vec3);

/// Базовый компонент отвечающий за основу [Entity]
#[derive(Component)]
pub struct EntityBase {
    pub speed: Speed,
    pub health: Health,
    pub position: Position,
    pub direction: DirectionState,
    pub velocity: Velocity,
    pub movable: bool,
}

impl Default for EntityBase {
    fn default() -> Self {
        Self {
            speed: Speed(50., 150., 25.),
            health: Health(1.),
            position: Position(Vec3::ZERO),
            direction: DirectionState::South,
            velocity: Velocity(Vec3::ZERO),
            movable: true,
        }
    }
}

#[derive(Component, Default)]
pub enum EntityState {
    #[default]
    Idle,
    Move,
}

#[derive(Component)]
pub enum EntityType {
    Humonoid(HumonoidType),
}

#[derive(Component, Default)]
pub enum EntityNeutrality {
    Hostile,
    Friendly,
    #[default]
    Neutral
}


#[derive(Component)]
pub enum HumonoidType {
    Human,
}

#[derive(Component, Default)]
pub enum EntityGender {
    Male,
    Female,
    Hermophrodite,
    #[default]
    None,
}

// #[derive(Event)]
// pub struct EntityCollisionEvent;

#[derive(Component)]
pub struct EntityMissile;

#[derive(Component)]
pub struct EntityParticle;
