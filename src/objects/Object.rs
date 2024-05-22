#![allow(unused)]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::{
    Entity::{
        Health,
        Position
    },
    Movement::DirectionState,
    resource::{
        SpriteLayer,
        Registry::Registry,
        graphic::Atlas::AtlasRes
    },
    world::chunk::Chunk::Chunk
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
            position: Position(Vec2::ZERO),
            direction: DirectionState::South,
            movable: true,
        }
    }
}

/// Событие спавна предмета
#[derive(Event)]
pub struct ObjectSpawn(pub String, pub IVec2);

/// Функция отвечающая за спавн предмета при вызове события спавна.
pub fn spawn_object(
    mut commands:   Commands,
    mut registry:   ResMut<Registry>,
    mut chunk_res:  ResMut<Chunk>,
        atlas:      Res<AtlasRes>,
    mut event:      EventReader<ObjectSpawn>
) {
    if event.is_empty() {
        return;
    }

    for event in event.read() {
        if !chunk_res.objects.contains_key(&event.1) {
            if let Some(info) = registry.get_object_info(&event.0) {
                if let Some(sprite) = registry.get_object_texture(&info.id_texture, &atlas) {
                    let entity = commands
                        .spawn((
                            EntityObject::default(),
                            SpriteSheetBundle {
                                texture: sprite.texture,
                                atlas: sprite.atlas,
                                transform: Transform {
                                    translation: Vec3::new(event.1.x as f32 * 16. + 8., event.1.y as f32 * 16. + 8., 0.8), // Откорректировать
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