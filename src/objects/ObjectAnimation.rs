use bevy::prelude::*;

#[derive(Component, Default, Debug, Reflect, PartialEq)]
pub enum ObjectDirectionState {
    #[default]
    South,
    North,
    West,
    East,
}