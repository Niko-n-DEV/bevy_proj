#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    AppState,
    UserSystem::UserControl,
    Object::EntityObject,
    Entity::EntityBase,
    // world::chunk::Chunk::Chunk,
    world::Grid::{
        Grid,
        get_format_current_chunk
    },
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
        mut objects:    Query<(Entity,&mut EntityObject)>,
        mut entities:   Query<(Entity, &Transform, &mut EntityBase), Without<UserControl>>,
        mut grid:       ResMut<Grid>,
        mut event:      EventReader<DamageObject>
    ) {
        if event.is_empty() {
            return;
        }

        for damage_event in event.read() {
            if let Some(chunk) = grid.chunks.get_mut(&get_format_current_chunk(damage_event.0)) {
                if let Some(object) = chunk.get_object(damage_event.0) {
                    if let Ok(mut entity) = objects.get_mut(object) {
                        if entity.1.health.0 > damage_event.1 {
                            entity.1.health.0 -= damage_event.1;
                        } else {
                            chunk.remove_object(entity.0);
                            commands.entity(entity.0).despawn_recursive();
                        }
                    }
                }
            }

            for mut entity in &mut entities {
                if 8.0 > Vec3::distance(damage_event.0.as_vec2().extend(0.5), entity.1.translation) {
                    if entity.2.health.0 > damage_event.1 {
                        entity.2.health.0 -= damage_event.1;
                    } else {
                        commands.entity(entity.0).despawn_recursive();
                    }
                }
            }
        }
    }
}