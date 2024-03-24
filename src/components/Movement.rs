#![allow(unused)]
use std::default;

use bevy::prelude::*;

#[derive(Component)]
struct Velocity {
    pub value: Vec3,
}

#[derive(Component, Default, Reflect)]
pub enum DirectionState {
    #[default]
    South,
    North,
    West,
    East
}