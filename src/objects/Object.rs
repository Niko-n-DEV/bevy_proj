#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    Entity::{
        Health,
        Position
    },
    Movement::DirectionState,
    resource::{
        Registry::Registry,
        graphic::Atlas::AtlasRes
    }
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

/// Событие спавна предмета
#[derive(Event)]
pub struct ObjectSpawn(pub String);

/// Функция отвечающая за спавн предмета при вызове события спавна.
pub fn spawn_object(
    mut commands:   Commands,
    mut registry:   ResMut<Registry>,
        atlas:      Res<AtlasRes>,
        event:      EventReader<ObjectSpawn>
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