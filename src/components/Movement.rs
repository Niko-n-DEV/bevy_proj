#![allow(unused)]
use std::default;

use bevy::prelude::*;

#[derive(Component, Default, Reflect, PartialEq)]
pub enum DirectionState {
    #[default]
    None,
    South,
    North,
    West,
    East
}