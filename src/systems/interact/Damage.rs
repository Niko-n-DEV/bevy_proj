#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    AppState,
    Object::EntityObject,
    world::chunk::Chunk::Chunk
};

pub struct DamageSystem;

impl Plugin for DamageSystem {
    fn build(&self, app: &mut App) {
        app
            // Init Events
            .add_event::<DamageObject>()
            // Init Systems
            .add_systems(Update, Self::damage_recorder.run_if(in_state(AppState::Game)))
        ;
    }
}

/// Ивент для нанесения урона объекту (1 - Местоположение | 2 - Урон)
#[derive(Event)]
pub struct DamageObject(pub IVec2, pub f32);

impl DamageSystem {
    fn damage_recorder(
        mut commands:   Commands,
        mut objects:    Query<(Entity, &mut EntityObject)>,
        mut chunk:      ResMut<Chunk>,
        mut event:      EventReader<DamageObject>
    ) {
        if event.is_empty() {
            return;
        }

        for damage_event in event.read() {
            if let Some(object) = chunk.objects.get(&damage_event.0) {
                if let Ok(mut entity) = objects.get_mut(*object) {
                    if entity.1.health.0 > damage_event.1 {
                        entity.1.health.0 -= damage_event.1;
                    } else {
                        println!("Entity: {} is destoyed!", entity.1.id_name);
                        commands.entity(entity.0).despawn_recursive();
                        chunk.objects.remove(&damage_event.0);
                    }
                } else {
                    warn!("Damage: Failed to get the entity!")
                }
            }
        }
    }
}