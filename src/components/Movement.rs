#![allow(unused)]
use std::default;

use bevy::prelude::*;

#[derive(Component, Default, Debug, Reflect, PartialEq)]
pub enum DirectionState {
    #[default]
    None,
    South,
    North,
    West,
    East,
}
