#![allow(unused)]
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::dynamics::RigidBodyBuilder};

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;


use crate::core::{
    resource::{
        Registry::{
            Registry,
            EntityRegistry
        },
        graphic::Atlas::AtlasRes,
        SpriteLayer
    },
    //UserSystem::User,
    EntityAnimation::EntityDirectionState,
    EntityType::*,
    entities::ai::{
        LastDirection,
        Path::AiPath,
    },
    stats::Stats,
};

/// Компонент отвечающий за [Здоровье]
#[derive(Component, Reflect, Default, Debug)]
pub struct Health(pub f32);

/// Компонент отвечающий за [Скорость]
///
/// 0 - Обычная скорость | 1 - Бег | 2 - Медленное передвижение
#[derive(Component, Reflect)]
pub struct Speed(pub f32, pub f32, pub f32);

/// Компонент отвечающий за [Позицию]
#[derive(Component, Reflect, Default, Debug)]
pub struct Position(pub Vec2);

/// Компонент отвечающий за [Взаимодействие]
/// 
/// 1 - Радиус взаимодействия / 2 - Радиус атаки
#[derive(Component, Reflect, Default, Debug)]
pub struct InteractionType(pub f32, pub f32);

/// Компонент отвечающий за возможность атаки на сущность.
/// 
/// При значении `false` не даёт наносить урон сущности, давая её неуязвимость.
#[derive(Component, Reflect)]
pub struct Attackable(pub bool);

/// Базовый компонент отвечающий за основу [Entity]
#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct EntityBase {
    pub id_name:            String,
    pub speed:              Speed,
    pub health:             Health,
    pub position:           Position,
    pub direction:          EntityDirectionState,
    pub movable:            bool,
    pub interaction_radius: f32,
    pub atack_radius:       f32,
    pub entity_type:        EntityType
}

impl Default for EntityBase {
    fn default() -> Self {
        Self {
            id_name:            "Unknown".to_string(),
            speed:              Speed(50., 75., 25.),
            health:             Health(1.),
            position:           Position(Vec2::ZERO),
            direction:          EntityDirectionState::South,
            movable:            true,
            interaction_radius: 10.0,
            atack_radius:       10.0,
            entity_type:        EntityType::None
        }
    }
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct EntityHead {
    pub id_name:    String,
    pub parent:     Entity,
    pub health:     Health,
    pub look_at:    Vec2,
    pub direction:  EntityDirectionState,
    pub movable:    bool,
}

/// Событие спавна сущности
#[derive(Event)]
pub struct EntitySpawn(pub String, pub Vec2);

/// Функция отвечающая за спавн сущности при вызове события спавна.
pub fn spawn_entity(
    mut commands:   Commands,
    mut registry:   ResMut<Registry>,
    mut event:      EventReader<EntitySpawn>,
        atlas:      Res<AtlasRes>,
) {
    if event.is_empty() {
        return;
    }

    for event in event.read() {
        if let Some(info) = registry.get_entity_info(&event.0) {
            if let Some(sprite_b) = registry.get_entity_texture(&info.id_texture_b, &atlas) {
                println!("Entity spawn: {} {}", event.0, event.1);
                let entity = commands.spawn((
                    RigidBody::Dynamic,
                    Damping {
                        linear_damping: 20.0,
                        ..default()
                    },
                    Velocity::zero(),
                    Collider::round_cuboid(3., 3., 0.25),
                    LockedAxes::ROTATION_LOCKED,
                    EntityBase {
                        id_name:    info.id_name.clone(),
                        speed:      Speed(50., 75., 25.),
                        health:     Health(info.health),
                        position:   Position(Vec2::new(64., 64.)),
                        direction:  EntityDirectionState::South,
                        movable:    true,
                        ..default()
                    },
                    SpriteSheetBundle {
                        texture: sprite_b.texture,
                        atlas: sprite_b.atlas,
                        transform: Transform {
                            translation: Vec3::new(event.1.x, event.1.y, 0.5),
                            ..default()
                        },
                        ..default()
                    },
                    SpriteLayer::Entity,
                    // Body,
                    info.entity_type.clone(),
                    EntityNeutrality::Neutral,
                    AiPath::default(),
                    LastDirection(Vec2::ZERO),
                    Stats::new(),
                    Name::new(info.id_name.clone()),
                )).id();

                if !info.id_texture_h.is_none() {
                    if let Some(texture_h) = info.id_texture_h.clone() {
                        if let Some(sprite_h) = registry.get_entity_texture(&texture_h, &atlas) {
                            commands.entity(entity).with_children(|parent| {
                                parent.spawn((
                                    EntityHead {
                                        id_name:    info.id_name.clone(),
                                        parent:     entity,
                                        health:     Health(info.health),
                                        look_at:    Vec2::ZERO,
                                        direction:  EntityDirectionState::South,
                                        movable:    true,
                                    },
                                    SpriteSheetBundle {
                                        texture: sprite_h.texture,
                                        atlas: TextureAtlas {
                                            layout: sprite_h.atlas.layout,
                                            index: sprite_h.atlas.index + 3
                                        },
                                        ..default()
                                    },
                                    SpriteLayer::EntityPart,
                                    //Head::default(),
                                    Name::new("Head"),
                                ));
                            });
                        }
                    }
                }
            } else {
                warn!("Error - Ошибка при попытке считывания текстуры из атласа.")
            }
        } else {
            warn!("Error - Ошибка при чтении информации из реестра, либо нет такой записи.")
        }
    }
    // commands.entity(entity)
    // .insert(User {
    //     control_entity: Some(entity),
    //     ..default()
    // })
    // .insert(Inventory::with_capacity(12));
}

/// Определяет состояние сущности [статичен, стоит или двигается]
#[derive(Component, Default)]
pub enum EntityState {
    Fixed,
    #[default]
    Idle,
    Move,
}

// #[derive(Component, Default)]
// pub struct Head {
//     pub look_at: Vec2
// }

// // Это для определения частей тела, чтобы к ним прикреплять одежду с контейнерами, вычислять показатели модульного здоровья
// #[derive(Component)]
// pub struct Body;

// Это нужно переделать/перенести от сюда
#[derive(Component)]
pub struct EntityMissile;

#[derive(Component)]
pub struct EntityParticle;
