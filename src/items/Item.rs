#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    resource::{
        graphic::Atlas::AtlasRes, 
        Registry::Registry
    }, 
    world::chunk::Chunk::Chunk, 
    Entity::Position, 
    Object::EntityObject,
    ItemType::{
        ItemEntity,
        ItemType,
        Weapon
    },
    Weapon::Gun,
};

//
//
//

#[derive(Component, Default)]
pub struct Durability(pub f32); 

//
//
//

#[derive(Component, Default)]
pub struct EntityItem {
    pub id_name:    String,
    pub name:       String,
    pub durability: Durability,
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
                            ItemEntity {
                                name:       info.id_name.clone(),
                                id_name:    info.id_name.clone(),
                                id_source:  info.id_source.clone(),
                                item_type:  info.item_type.clone(),
                                durability: info.durability.clone(),
                                stack_size: info.stack_size.clone(),
                                stackable:  info.stackable.clone(),
                                count:      event.2,
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
                            Name::new(info.id_name.clone())
                        )).id();

                        match info.item_type {
                            (ItemType::Item(_) | ItemType::None) => {
                                // По идеи тут ничего
                            },
                            ItemType::Weapon(_) => {
                                if let Some(var) = info.range_info {
                                    commands.entity(entity).insert(Gun {
                                        shoot_cooldown:     var.0,
                                        bullet_lifetime:    var.1,
                                        shoot_timer:        var.2
                                    });
                                }
                            },
                            ItemType::Tool(_) => {
                                
                            }
                        }
    
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