use bevy::prelude::*;

use crate::core::{
    Entity::{
        Health,
        Position
    },
    Movement::DirectionState
};

#[derive(Component)]
pub struct EntityObject {
    pub health: Health,
    pub position: Position,
    pub direction: DirectionState,
    pub movable: bool,
}

impl Default for EntityObject {
    fn default() -> Self {
        Self {
            health: Health(1.),
            position: Position(Vec3::ZERO),
            direction: DirectionState::South,
            movable: true,
        }
    }
}