#![allow(unused)]
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use super::Entity::EntityBase;

//
//
//

#[derive(Event)]
pub struct DeshEvent(pub Entity);

fn dash_skill(
    mut entity: Query<(
        &EntityBase, 
        &mut Velocity, 
        &mut Transform
    )>,
    mut event: EventReader<DeshEvent>
) {
    if event.is_empty() {
        return;
    }

    for event in event.read() {
        if let Ok(entity) = entity.get_mut(event.0) {

        }
    }
}