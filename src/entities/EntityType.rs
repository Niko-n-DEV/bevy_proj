#![allow(unused)]
use bevy::prelude::*;

use serde::{Deserialize, Serialize};

// будет корректировка
/// Тип сущности 
#[derive(Component, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum EntityType {
    None,
    Humonoid(HumonoidType),
    Animal
}

/// Какого типа гумонойд (помимо человека будут и другие расы)
#[derive(Component, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum HumonoidType {
    Human,
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

/// Гендер существа
#[derive(Component, Default)]
pub enum EntityGender {
    Male,
    Female,
    Hermophrodite,
    #[default]
    None,
}