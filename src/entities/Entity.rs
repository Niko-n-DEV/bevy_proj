#![allow(unused)]
use std::default;

use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    resource::{
        Registry::{
            Registry,
            EntityRegistry
        },
        graphic::Atlas::AtlasRes
    },
    UserSystem::User,
    Movement::DirectionState
};

/// Компонент отвечающий за [Здоровье]
#[derive(Component, Reflect)]
pub struct Health(pub f32);

/// Компонент отвечающий за [Скорость]
///
/// 0 - Обычная скорость | 1 - Бег | 2 - Медленное передвижение
#[derive(Component, Reflect)]
pub struct Speed(pub f32, pub f32, pub f32);

/// Компонент отвечающий за [Позицию]
#[derive(Component, Reflect)]
pub struct Position(pub Vec3);

/// Компонент отвечающий за [Направление движения]
// #[derive(Component, Reflect)]
// pub struct Velocity(pub Vec3);

/// Компонент отвечающий за возможность атаки на сущность.
/// 
/// При значении `false` не даёт наносить урон сущности, давая её неуязвимость.
#[derive(Component, Reflect)]
pub struct Attackable(pub bool);

/// Базовый компонент отвечающий за основу [Entity]
#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct EntityBase {
    pub speed: Speed,
    pub health: Health,
    pub position: Position,
    pub direction: DirectionState,
    pub movable: bool,
    pub interaction_radius: f32,
    pub entity_type: EntityType
}

impl Default for EntityBase {
    fn default() -> Self {
        Self {
            speed: Speed(50., 75., 25.),
            health: Health(1.),
            position: Position(Vec3::ZERO),
            direction: DirectionState::South,
            movable: true,
            interaction_radius: 10.0,
            entity_type: EntityType::None
        }
    }
}

/// Событие спавна сущности
#[derive(Event)]
pub struct EntitySpawn(pub String);

/// Функция отвечающая за спавн сущности при вызове события спавна.
pub fn spawn_entity(
    mut commands:   Commands,
    mut registry:   ResMut<Registry>,
        atlas:      Res<AtlasRes>,
        event:      EventReader<EntitySpawn>
) {
    if event.is_empty() {
        return;
    }


    // commands.spawn((
    //     RigidBody::Dynamic,
    //     EntityBase {
    //         speed: Speed(50., 75., 25.),
    //         health: Health(100.),
    //         position: Position(Vec3::new(64., 64., 0.)),
    //         direction: DirectionState::South,
    //         movable: true,
    //         ..default()
    //     },
    //     sprite,
    //     EntityType::Humonoid(HumonoidType::Human),
    //     EntityNeutrality::Neutral,
    //     Name::new("Player"),
    // ));
}

// Добавил чисто для теста
#[derive(Bundle)]
pub struct EntityFounder {
    pub health: Health,
    pub speed: Speed,
    pub position: Position,
    pub direction: DirectionState,
    pub entity_type: EntityType
}

/// Определяет состояние сущности [статичен, стоит или двигается]
#[derive(Component, Default)]
pub enum EntityState {
    Fixed,
    #[default]
    Idle,
    Move,
}

// будет корректировка
/// Тип сущности 
#[derive(Component, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum EntityType {
    None,
    Humonoid(HumonoidType),
    Animal
}

// будет переделываться на систему репутации и хищничества
/// Поведение в отношении игрока
#[derive(Component, Default)]
pub enum EntityNeutrality {
    Hostile,
    Friendly,
    #[default]
    Neutral
}

/// Какого типа гумонойд (помимо человека будут и другие расы)
#[derive(Component, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum HumonoidType {
    Human,
}

// Это для определения частей тела, чтобы к ним прикреплять одежду с контейнерами, вычислять показатели модульного здоровья
#[derive(Component)]
pub struct Body;

/// Гендер существа
#[derive(Component, Default)]
pub enum EntityGender {
    Male,
    Female,
    Hermophrodite,
    #[default]
    None,
}

// Это нужно переделать/перенести от сюда
#[derive(Component)]
pub struct EntityMissile;

#[derive(Component)]
pub struct EntityParticle;
