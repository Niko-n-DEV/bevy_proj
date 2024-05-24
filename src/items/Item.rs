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
    ItemType::ItemEntity
};

#[derive(Component, Default)]
pub struct EntityItem {
    pub id_name:    String,
    pub name:       String,
    pub health:     Health,
    pub position:   Position,
}

/// Событие спавна предмета
#[derive(Event)]
pub struct ItemSpawn(pub String, pub IVec2, pub usize);

/// Функция отвечающая за спавн предмета при вызове события спавна.
pub fn spawn_item(
    mut commands:   Commands,
        registry:   Res<Registry>,
    mut chunk_res:  ResMut<Chunk>,
        atlas:      Res<AtlasRes>,
    mut items:      Query<(Entity, &mut ItemEntity), With<ItemEntity>>,
    mut event:      EventReader<ItemSpawn>
) {
    if event.is_empty() {
        return;
    }

    for event in event.read() {
        if !chunk_res.objects_ex.contains_key(&event.1) {
            if let Some(info) = registry.get_item_info(&event.0) {
                if let Some(sprite) = registry.get_item_texture(&info.id_texture, &atlas) {
                    let entity = commands
                    .spawn((
                        EntityItem {
                            id_name: info.id_name.clone(),
                            name: info.id_name.clone(),
                            ..default()
                        },
                        SpriteSheetBundle {
                            texture: sprite.texture,
                            atlas: sprite.atlas,
                            transform: Transform {
                                translation: Vec3::new(event.1.x as f32 * 8. + 4., event.1.y as f32 * 8. + 4., 0.3),
                                scale: Vec3::new(0.5, 0.5, 0.0),
                                ..default()
                            },
                            ..default()
                        },
                        ItemEntity {
                            item: info.item_type,
                            count: event.2
                        },
                        Name::new(info.id_name.clone())
                    )).id();
    
                    chunk_res.objects_ex.insert(event.1, entity);
                }
            }
        } else {
            if let Some(sub_obj_entity) = chunk_res.objects_ex.get(&event.1) {
                if let Ok(mut entity) = items.get_mut(*sub_obj_entity) {
                    entity.1.count += event.2;
                }
            }
        }
    }
}