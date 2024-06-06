use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::{
    resource::{
        graphic::{
            Atlas::AtlasRes, Connect::ConnectedObject
        }, Registry::Registry, SpriteLayer
    }, 
    world::{
        World::*,
        chunk::Chunk::Chunk, 
        Grid::*
    }, 
    Entity::{
        Health,
        Position
    }, 
    ObjectAnimation::ObjectDirectionState
};

// ====================
// Entity Object
// ====================

#[allow(unused)]
#[derive(Component)]
pub struct EntityObject {
    pub id_name:    String,
    pub health:     Health,
    pub position:   Position,
    pub direction:  ObjectDirectionState,
    pub movable:    bool,
}

impl Default for EntityObject {
    fn default() -> Self {
        Self {
            id_name:    "Object".to_string(),
            health:     Health(100.),
            position:   Position(Vec2::ZERO),
            direction:  ObjectDirectionState::South,
            movable:    true,
        }
    }
}

// ====================
// Entity Persistent Object
// ====================

#[allow(unused)]
#[derive(Component)]
pub struct PersistentObject {
    pub id_name:    String,
    pub health:     Health,
    pub position:   Position,
    pub direction:  ConnectedObject,
}

impl Default for PersistentObject {
    fn default() -> Self {
        Self {
            id_name:    "Object".to_string(),
            health:     Health(2.),
            position:   Position(Vec2::ZERO),
            direction:  ConnectedObject::South
        }
    }
}

// ======================
// Entity Objects Spawner
// ======================

/// Событие спавна объекта
#[derive(Event)]
pub struct ObjectSpawn(pub String, pub IVec2);

/// Функция отвечающая за спавн объекта при вызове события спавна.
pub fn spawn_object(
    mut commands:   Commands,
        registry:   Res<Registry>,
    mut grid:       ResMut<Grid>,
        atlas:      Res<AtlasRes>,
    mut event:      EventReader<ObjectSpawn>
) {
    if event.is_empty() {
        return;
    }

    for event in event.read() {
        if let Some(info) = registry.get_object_info(&event.0) {
            if let Some(sprite) = registry.get_object_texture(&info.id_texture, &atlas) {
                let coord = WorldSystem::get_currect_chunk_tile(event.1);

                let entity = commands.spawn((
                    EntityObject {
                        id_name: info.id_name.clone(),
                        health:  Health(info.health.clone() as f32), 
                        ..default()
                    },
                    SpriteSheetBundle {
                        texture: sprite.texture,
                        atlas: sprite.atlas,
                        transform: Transform {
                            translation:    Vec3::new(coord.x as f32 * 16.0 + 8.0, coord.y as f32 * 16.0 + 8.0, 0.8), // Откорректировать
                            scale:          Vec3::splat(0.5),
                            ..default()
                        },
                        ..default()
                    },
                    SpriteLayer::Object,
                    RigidBody::Fixed,
                    Collider::cuboid(info.collision.x, info.collision.y),
                    Name::new(info.id_name.clone())
                )).id();

                if !grid.add_object_to_chunk(entity, event.1) {
                    // println!("Object {} been deleted, due to an installation error!", &event.0);
                    commands.entity(entity).despawn();
                }
            } else {
                warn!("В атласе не была найдена текстура для - {}", &event.0);
                break;
            }
        } else {
            warn!("В регистре не найден объект - {}", &event.0);
            break;
        }
    }
}

#[allow(unused)]
/// Событие спавна постоянного объекта
#[derive(Event)]
pub struct PersistentObjectSpawn(pub String, pub IVec2);

#[allow(unused)]
/// Функция отвечающая за спавн постоянного объекта при вызове события спавна.
pub fn spawn_persistent_object(
    mut commands:   Commands,
        registry:   Res<Registry>,
    mut chunk_res:  ResMut<Chunk>,
        atlas:      Res<AtlasRes>,
    mut event:      EventReader<PersistentObjectSpawn>
) {
    if event.is_empty() {
        return;
    }

    for event in event.read() {
        if !chunk_res.objects.contains_key(&event.1) {
            if let Some(info) = registry.get_object_ct_info(&event.0) {
                if let Some(sprite) = registry.get_object_ct_texture(&info.id_texture, &atlas) {
                    let entity = commands
                        .spawn((
                            PersistentObject {
                                id_name: info.id_name.clone(),
                                ..default()
                            },
                            SpriteSheetBundle {
                                texture: sprite.texture,
                                atlas: sprite.atlas,
                                transform: Transform {
                                    translation:    Vec3::new(event.1.x as f32 * 16. + 8., event.1.y as f32 * 3162. + 8., 0.8), // Откорректировать
                                    scale:          Vec3::splat(0.5),
                                    ..default()
                                },
                                ..default()
                            },
                            SpriteLayer::Object,
                            RigidBody::Fixed,
                            Collider::cuboid(info.collision.x, info.collision.y),
                            Name::new(info.id_name.clone())
                        )).id();
                    
                    chunk_res.objects.insert(event.1, entity);
                }
            }
        }
    }
}