use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use crate::core::{
    resource::{
        graphic::Atlas::{
            AtlasType,
            AtlasRes,
        }, 
        Registry::Registry
    }, 
    world::chunk::Chunk::Chunk, 
    Entity::EntityBase, 
    // Object::EntityObject,
    ItemType::{
        ItemType,
        // Weapon
    },
    Weapon::Gun,
    ContainerSystem::Inventory,
    AppState,
};

use super::ItemType::ItemStackType;

//
//
//

pub fn item_plugin(app: &mut App) {
    // Types
    app.register_type::<ItemEntity>();
    // Events
    app.add_event::<ItemSpawn>();
    app.add_event::<TakeItem>();
    // Systems
    app.add_systems(FixedUpdate, (spawn_item, take_item).run_if(in_state(AppState::Game)));
}

//
//
//

#[allow(unused)]
#[derive(Component, Default)]
pub struct Durability(pub f32); 

/// Структура для того, что может быть поднято
#[derive(Component, InspectorOptions, Default, Debug, Clone, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct ItemEntity {
    pub name:       String,
    pub id_name:    String,
    pub id_source:  Option<String>,
    pub item_type:  ItemType,
    pub durability: Option<usize>,
    pub stack_size: Option<usize>,
    pub stackable:  Option<ItemStackType>,
    pub count:      usize
}

impl ItemEntity {
    pub fn check_stackable(&self) -> bool {
        if let Some(stack) = self.stackable {
            return stack.is_stackable()
        }
        false
    }
}

//
//
//

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
                match info.item_type {

                    ItemType::Item(_) | ItemType::None => {
                        if let Some(sprite) = registry.get_item_texture(&info.id_texture, &atlas, AtlasType::Items) {
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

                            chunk_res.objects_ex.insert(event.1, entity);
                        }
                    },

                    ItemType::Weapon(_) => {
                        if let Some(sprite) = registry.get_item_texture(&info.id_texture, &atlas, AtlasType::Weapon) {
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
                            
                            if let Some(var) = info.range_info {
                                commands.entity(entity).insert(Gun {
                                    shoot_cooldown:     var.0,
                                    bullet_lifetime:    var.1,
                                    shoot_timer:        var.2
                                });
                            }

                            chunk_res.objects_ex.insert(event.1, entity);
                        }
                    },

                    ItemType::Tool(_) => {
                        
                    }
                }
            }
        } else {
            if let Some(sub_obj_entity) = chunk_res.objects_ex.get(&event.1) {
                if let Ok(mut entity) = items.get_mut(*sub_obj_entity) {
                    if entity.1.check_stackable() {
                        if let Some(stack_size) = entity.1.stack_size {
                            if (entity.1.count + event.2) > stack_size {
                                entity.1.count = stack_size;
                            } else {
                                entity.1.count = entity.1.count + event.2;
                            }
                        } else {
                            entity.1.count += event.2;
                        }
                    }
                }
            }
        }
    }
}

//
// Take
//

/// Событие взятие в инвентарь предмета
#[derive(Event)]
pub struct TakeItem(pub Entity, pub Option<Entity>); // 1 - Местоположение источника вызова

pub fn take_item(
    mut commands: Commands,
    mut event:    EventReader<TakeItem>,
    mut chunk:    ResMut<Chunk>,
    mut entity:   Query<(&mut Inventory, &EntityBase)>,  
    mut item_q:   Query<(Entity, &Transform, &mut ItemEntity)>    
) {
    if event.is_empty() && (item_q.is_empty() || entity.is_empty()) {
        return;
    }

    for event in event.read() {
        // Берёт сущность, которая вызывает поднятие предмета
        if let Ok(mut customer) = entity.get_mut(event.0) {
            if let Some(source) = event.1 {
                // Если есть источник, то проверяем расстояние до него
                if let Ok(mut item) = item_q.get_mut(source) {
                    if customer.1.interaction_radius > Vec3::distance(item.1.translation, customer.1.position.0.extend(0.5)) {
                        println!("{:?}", item.2);
                        if customer.0.add(&mut item.2) {
                            println!("{:?}", item.2);
                            if item.2.count == 0 {
                                chunk.remove_sub_object_ex(item.0);
                                commands.entity(item.0).despawn_recursive();
                            }
                        }
                    }
                }
            } else {
                for mut item in &mut item_q {
                    if customer.1.interaction_radius > Vec3::distance(item.1.translation, customer.1.position.0.extend(0.5)) {
                        if customer.0.add(&mut item.2) {
                            if item.2.count == 0 {
                                chunk.remove_sub_object_ex(item.0);
                                commands.entity(item.0).despawn_recursive();
                            }
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
    }
}