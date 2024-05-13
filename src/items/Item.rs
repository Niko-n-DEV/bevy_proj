#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    Entity::{
        Health,
        Position
    },
    resource::{
        Registry::Registry,
        graphic::Atlas::AtlasRes
    }
};

#[derive(Component)]
pub struct EntityItem {
    pub health: Health,
    pub position: Position,
}

/// Событие спавна предмета
#[derive(Event)]
pub struct ItemSpawn(pub String);

/// Функция отвечающая за спавн предмета при вызове события спавна.
pub fn spawn_item(
    mut commands:   Commands,
    mut registry:   ResMut<Registry>,
        atlas:      Res<AtlasRes>,
        event:      EventReader<ItemSpawn>
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