#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    resource::{
        graphic::Atlas::AtlasRes, 
        Registry::Registry
    }, 
    world::chunk::Chunk::Chunk, 
    Entity::{
        Health,
        Position
    }, 
    Object::EntityObject,
    ItemType::Pickupable
};

#[derive(Component)]
pub struct EntityItem {
    pub health: Health,
    pub position: Position,
}

/// Событие спавна предмета
#[derive(Event)]
pub struct ItemSpawn(pub String, pub IVec2, pub usize);

/// Функция отвечающая за спавн предмета при вызове события спавна.
pub fn spawn_item(
    mut commands:   Commands,
    mut registry:   ResMut<Registry>,
    mut chunk_res:  ResMut<Chunk>,
        atlas:      Res<AtlasRes>,
    mut event:      EventReader<ItemSpawn>
) {
    if event.is_empty() {
        return;
    }

    for event in event.read() {
        if let Some(sprite) = registry.get_item(&event.0, &atlas) {
            if let Some(info) = registry.get_item_info(&event.0) {
                let entity = commands
                .spawn((
                    EntityObject::default(),
                    SpriteSheetBundle {
                        texture: sprite.texture,
                        atlas: sprite.atlas,
                        transform: Transform {
                            translation: Vec3::new(event.1.x as f32 * 16. + 8., event.1.y as f32 * 16. + 8., 0.3),
                            ..default()
                        },
                        ..default()
                    },
                    Pickupable {
                        item: info.item_type,
                        count: event.2
                    },
                    Name::new(info.id_name.clone())
                )).id();

                chunk_res.objects_ex.insert(event.1, entity);
            }
        }
    }
}