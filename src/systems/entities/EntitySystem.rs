#![allow(unused)]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::
    core::{
        UserSystem::{
            UserControl,
            User,
        },
        resource::{
            SpriteLayer,
            graphic::Atlas::{
                AtlasRes,
                DirectionAtlas, 
                TestTextureAtlas
            },
            Registry::Registry
        },
        Entity::{
            EntityBase,
            EntityHead,
            Health,
            Position,
        },
        EntityType::{
            EntityType,
            HumonoidType,
            EntityNeutrality
        },
        Movement::DirectionState,
        Missile::{update_bullet_hits, update_bullets},
        AppState
    };

// ==============================
// TEST
// Enemy
// ==============================

#[derive(Component)]
pub struct EnemySpawner {
    pub is_active: bool,
    pub cooldown: f32,
    pub timer: f32,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            is_active:  false,
            cooldown:   1.,
            timer:      1.
        }
    }
}

#[derive(Component)]
pub struct Enemy;

pub fn update_spawning(
    mut spawner_query: Query<(&mut EnemySpawner, &Transform)>,
    time: Res<Time>,
    handle: Res<TestTextureAtlas>,
    mut commands: Commands,
) {
    for (mut spawner, trans) in spawner_query.iter_mut() {
        if spawner.is_active {
            spawner.timer -= time.delta_seconds();
            if spawner.timer <= 0. {

                spawner.timer = spawner.cooldown;
                let pos = Vec3::new(trans.translation.x, trans.translation.y, 0.5);

                commands
                    .spawn(SpriteSheetBundle {
                        texture: handle.image.clone().unwrap(),
                        atlas: TextureAtlas {
                            layout: handle.layout.clone().unwrap(),
                            index: TestTextureAtlas::get_index("mob", &handle),
                        },
                        transform: Transform {
                            translation: pos,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(EntityBase {
                        id_name: "human".to_string(),
                        health: Health(50.0),
                        direction: DirectionState::South,
                        //velocity: Velocity(Vec3::ZERO),
                        movable: true,
                        ..default()
                    })
                    .insert(Enemy)
                    .insert((
                        EntityType::Humonoid(HumonoidType::Human),
                        EntityNeutrality::Hostile,
                    ))
                    .insert(SpriteLayer::Entity)
                    .insert(Velocity::zero())
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::round_cuboid(2., 2., 0.25))
                    .insert(LockedAxes::ROTATION_LOCKED);
            }
        }
    }
}

// ==================================================
// -= Test =-
// Обновление врагов
// В данном случае движение их в сторону позиции игрока
// ==================================================
pub fn update_enemies(
    mut commands:       Commands,
    mut enemy_query:    Query<(&mut Transform, &EntityBase, Entity), With<Enemy>>,
        player_query:   Query<(&Transform, &UserControl), Without<Enemy>>,
    mut move_event:     EventWriter<MovementEntity>
) {
    if enemy_query.is_empty() || player_query.is_empty() {
        return;
    }

    if let Ok((player_transform, _)) = player_query.get_single() {
        for (transform, enemy, entity) in enemy_query.iter_mut() {
            let direction = Vec3::normalize(player_transform.translation - transform.translation);

            move_event.send(MovementEntity(entity, direction.normalize(), enemy.speed.0));

            if enemy.health.0 <= 0. {
                commands.entity(entity).despawn();
            }
        }
    } else {
        warn!("Error - An exception occurred while reading player_query!")
    }
}

// ==============================
// EntitySystem
// ==============================

pub struct EntitySystem;

impl Plugin for EntitySystem {
    fn build(&self, app: &mut App) {
        app
            // Init Register
            .register_type::<EntityBase>()
            .register_type::<EntityHead>()
            // Init Events
            .add_event::<DirectionChangeEvent>()
            .add_event::<MovementEntity>()
            // Init Plugins
            
            // Init Systems
            .add_systems(
                Update,
                (
                    handle_move, // .before(PhysicsSet::SyncBackend),
                    // handle_direction_changed_events.after(handle_move),
                    change_dir_velocity,
                    change_dir_head
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(PostUpdate, inertia_attenuation.run_if(in_state(AppState::Game)))
            // [Test] Обновление системы просчёта пуль и попадений
            .add_systems(
                Update,
                (
                    update_bullets,
                    update_bullet_hits
                ).run_if(in_state(AppState::Game))
            )
            // [Test] Обновление системы просчёта врагов и их спавна
            .add_systems(
                Update,
                (
                    update_enemies,
                    update_spawning
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(OnExit(AppState::Game), delete_enemy_spawner)
        ;
    }
}



// Перенести в world, т.к. эdSAтот компонет спавнера
/// Удаление спавнера врагов при выходе из сцены игры
fn delete_enemy_spawner(
    mut commands: Commands,
    spawner: Query<Entity, With<EnemySpawner>>,
) {
    if spawner.is_empty() { // && enemy.is_empty() {
        return;
    }

    if let Ok(spawner) = spawner.get_single() {
        commands.entity(spawner).despawn_recursive()
    }
}

// ==============================
// Movement
// ==============================

#[derive(Event)]
pub struct MovementEntity(pub Entity, pub Vec3, pub f32); // 0 - Entity, 1 - Diraction, 2 - Speed

fn handle_move(
    mut query: Query<(
        &mut EntityBase, 
        &mut Transform,
        &mut Velocity
    )>,
    mut event:      EventReader<MovementEntity>,
    // mut dir_event:  EventWriter<DirectionChangeEvent>,
    // time: Res<Time>
) {
    if event.is_empty() {
        return;
    }

    for event in event.read() {
        if let Ok((mut entity_base, mut transform, mut velocity)) = query.get_mut(event.0) {
            if event.1 != Vec3::ZERO {
                //dir_event.send(DirectionChangeEvent(event.0, determine_direction_vec3(event.1)));
                //transform.translation = transform.translation + time.delta_seconds() * event.2 * event.1;
                
                let move_var = event.1 / event.1.length();
                velocity.linvel = move_var.truncate() * event.2;

                entity_base.position = Position(transform.translation.truncate());
            } else {
                transform.translation = entity_base.position.0.extend(0.5)
            }
        }
    }
}

// Гаситель инерции всех движиющийся Entity
fn inertia_attenuation(
    mut query: Query<&mut Velocity, With<EntityBase>>,
) {
    if query.is_empty() {
        return;
    }

    let damping_coefficient = 0.75;
    let min_threshold = 0.01;

    for mut vel in query.iter_mut() {
        if vel.linvel != Vec2::ZERO {
            vel.linvel *= damping_coefficient;
            
            if vel.linvel.abs().max_element() < min_threshold {
                vel.linvel = Vec2::ZERO;
            }
        }
    }
}


// /// Функция для определения направления на основе нормализованного Vec3 вектора
// fn determine_direction_vec3(vector: Vec3) -> DirectionState {
//     if vector == Vec3::ZERO {
//         return DirectionState::None;
//     }

//     let angle = vector.y.atan2(vector.x).to_degrees().rem_euclid(360.0);

//     if angle > 45.0 && angle <= 135.0 {
//         return DirectionState::North;
//     }
//     if angle > 135.0 && angle <= 225.0 {
//         return DirectionState::West;
//     }
//     if angle > 225.0 && angle <= 315.0 {
//         return DirectionState::South;
//     }
//     DirectionState::East
// }

/// Функция для определения направления на основе нормализованного Vec2 вектора
fn determine_direction_vec2(vector: Vec2) -> Option<DirectionState> {
    if vector == Vec2::ZERO {
        return None;
    }

    let angle = vector.y.atan2(vector.x).to_degrees().rem_euclid(360.0);

    if angle > 45.0 && angle <= 135.0 {
        return Some(DirectionState::North);
    }
    if angle > 135.0 && angle <= 225.0 {
        return Some(DirectionState::West);
    }
    if angle > 225.0 && angle <= 315.0 {
        return Some(DirectionState::South);
    } 
    if (angle > 315.0 && angle <= 360.0) || (angle >= 0.0 && angle <= 45.0) {
        return Some(DirectionState::East);
    }

    None
}

// Direction texture updater
#[derive(Event)]
pub struct DirectionChangeEvent(pub Entity, pub DirectionState);

// Важно создать систему, которая будет контролировать все сущности
/*
    Сущность перемещается, при изменение направления движения изменяется атрибут направления, когда атрибут направления изменён происходит event,
    который получает ссылку на entity и изменённое направление. Когда ивент создан, наверно, происходит выполнения функции, в котором ивент считывается
*/

// скорее будет работать по ивенту, по типу if direction_entity_is_change -> изменение текстуры на другое направление
/// Обновляет текстуру моба в зависимости от его направления
// fn handle_direction_changed_events(
//     mut _query: Query<(
//         &mut EntityBase, 
//         &mut TextureAtlas,
//         &EntityType
//     )>,
//     _handle_dir:    Res<DirectionAtlas>,
//     mut event:      EventReader<DirectionChangeEvent>,
// ) {
//     if event.is_empty() {
//         return;
//     }

//     let index_atlas = DirectionAtlas::get_index("human", &_handle_dir);
//     for event in event.read() {
//         if let Ok((mut _entity_base, mut atlas, _entity_type)) = _query.get_mut(event.0) {
//             match event.1 {
//                 DirectionState::North => {
//                     _entity_base.direction = DirectionState::North;
//                     atlas.index = index_atlas + 1;
//                 }
//                 DirectionState::South => {
//                     _entity_base.direction = DirectionState::South;
//                     atlas.index = index_atlas;
//                 }
//                 DirectionState::West => {
//                     _entity_base.direction = DirectionState::West;
//                     atlas.index = index_atlas + 3;
//                 }
//                 DirectionState::East => {
//                     _entity_base.direction = DirectionState::East;
//                     atlas.index = index_atlas + 2;
//                 }
//                 DirectionState::None => {
//                     _entity_base.direction = DirectionState::None;
//                 }
//             }
//         }
//     }
// }

pub fn change_dir_velocity(
    mut query: Query<(
        &mut EntityBase,
        &mut TextureAtlas,
        &mut Transform,
        &mut Velocity
    ), (Changed<Velocity>, With<EntityBase>)>,

    atlas:      Res<AtlasRes>,
    register:   Res<Registry>
) {
    if query.is_empty() {
        return;
    }

    for mut entity in &mut query {
        if let Some(hash) = &atlas.entity.ids {
            if let Some(info) = register.get_entity_info(&entity.0.id_name) {
                if let Some(index) = hash.get(&info.id_texture_b) {
                    match determine_direction_vec2(entity.3.linvel) {
                        Some(DirectionState::South) => {
                            entity.0.direction = DirectionState::South;
                            entity.1.index = *index;
                        }
                        Some(DirectionState::North) => {
                            entity.0.direction = DirectionState::North;
                            entity.1.index = index + 1
                        }
                        Some(DirectionState::East) => {
                            entity.0.direction = DirectionState::East;
                            entity.1.index = index + 2;
                        }
                        Some(DirectionState::West) => {
                            entity.0.direction = DirectionState::West;
                            entity.1.index = index + 3;
                        }
                        _ => {
                            return;
                        }
                    }
                }
            }
        }
    }
}


pub fn change_dir_head(
    // mut gizmos:     Gizmos,
    // Сущность которая меняет направление в зависимости от 
    mut query_h:    Query<(&mut EntityHead, &mut TextureAtlas), With<EntityHead>>,
    mut query_b:    Query<(Entity, &EntityBase, &Transform), With<EntityBase>>,
        atlas:      Res<AtlasRes>,
        register:   Res<Registry>
) {
    if query_h.is_empty() {
        return;
    }

    for mut entity_h in &mut query_h {
        for entity_b in &query_b {
            if entity_h.0.parent == entity_b.0 {
                // gizmos.line_2d(entity_b.1.translation.truncate(), entity_h.0.look_at, Color::YELLOW);
                // let dir: DirectionState =  determine_direction_vec2(entity_h.0.look_at - entity_b.2.translation.truncate());
                // println!("{:?}", dir)

                if let Some(hash) = &atlas.entity.ids {
                    if let Some(info) = register.get_entity_info(&entity_h.0.id_name) {
                        if let Some(texture_h) = &info.id_texture_h {
                            if let Some(index) = hash.get(texture_h) {
                                match determine_direction_vec2(entity_h.0.look_at - entity_b.2.translation.truncate()) {
                                    Some(DirectionState::South) => {
                                        if entity_b.1.direction != DirectionState::North {
                                            entity_h.0.direction = DirectionState::South;
                                            entity_h.1.index = index + 3;
                                        }
                                    }
                                    Some(DirectionState::North) => {
                                        if entity_b.1.direction != DirectionState::South {
                                            entity_h.0.direction = DirectionState::North;
                                            entity_h.1.index = index + 4
                                        }
                                    }
                                    Some(DirectionState::East) => {
                                        if entity_b.1.direction != DirectionState::West {
                                            entity_h.0.direction = DirectionState::East;
                                            entity_h.1.index = index + 5;
                                        }
                                        
                                    }
                                    Some(DirectionState::West) => {
                                        if entity_b.1.direction != DirectionState::East {
                                            entity_h.0.direction = DirectionState::West;
                                            entity_h.1.index = index + 6;
                                        }
                                        
                                    }
                                    _ => {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}