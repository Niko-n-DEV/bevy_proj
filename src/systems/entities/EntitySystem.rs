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
        EntityAnimation::EntityDirectionState,
        stats::Stats,
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
                        direction: EntityDirectionState::South,
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
            .register_type::<Stats>()
            // Init Events
            .add_event::<DirectionChangeEvent>()
            .add_event::<MovementEntity>()
            // Init Plugins
            
            // Init Systems
            .add_systems(
                Update,
                (
                    handle_move.before(PhysicsSet::SyncBackend),
                    // handle_direction_changed_events.after(handle_move),
                    change_dir_velocity,
                    change_dir_head
                ).run_if(in_state(AppState::Game))
            )
            // .add_systems(PostUpdate, inertia_attenuation.run_if(in_state(AppState::Game)))
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
    mut commands:   Commands,
        spawner:    Query<Entity, With<EnemySpawner>>,
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

/// Ивент для управления движением сущности
#[derive(Event)]
pub struct MovementEntity(pub Entity, pub Vec3, pub f32); // 0 - Entity, 1 - Diraction, 2 - Speed

fn handle_move(
    mut query: Query<(
        &mut EntityBase, 
        &mut Transform,
        &mut Velocity
    )>,
    mut event:      EventReader<MovementEntity>,
) {
    if event.is_empty() {
        return;
    }

    for event in event.read() {
        if let Ok((mut entity_base, mut transform, mut velocity)) = query.get_mut(event.0) {
            if event.1 != Vec3::ZERO {
                let move_var = event.1 / event.1.length();
                velocity.linvel = move_var.truncate() * event.2;

                entity_base.position = Position(transform.translation.truncate());
            }
            // else {
            //     transform.translation = entity_base.position.0.extend(0.5)
            // }
        }
    }
}

// Гаситель инерции всех движиющийся Entity
fn inertia_attenuation(
    mut query: Query<(&mut Velocity, &mut EntityBase, &Transform), With<EntityBase>>,
) {
    if query.is_empty() {
        return;
    }

    let damping_coefficient = 0.75;
    let min_threshold = 0.01;

    for mut vel in query.iter_mut() {
        if vel.0.linvel != Vec2::ZERO {
            vel.0.linvel *= damping_coefficient;
            
            if vel.0.linvel.abs().max_element() < min_threshold {
                vel.0.linvel = Vec2::ZERO;
                vel.1.position.0 = vel.2.translation.truncate()
            }
        }
    }
}

/// Функция для определения направления на основе нормализованного Vec2 вектора
fn determine_direction_vec2(vector: Vec2) -> Option<EntityDirectionState> {
    if vector == Vec2::ZERO {
        return None;
    }

    let angle = vector.y.atan2(vector.x).to_degrees().rem_euclid(360.0);

    if angle > 45.0 && angle <= 135.0 {
        return Some(EntityDirectionState::North);
    }
    if angle > 135.0 && angle <= 225.0 {
        return Some(EntityDirectionState::West);
    }
    if angle > 225.0 && angle <= 315.0 {
        return Some(EntityDirectionState::South);
    } 
    if (angle > 315.0 && angle <= 360.0) || (angle >= 0.0 && angle <= 45.0) {
        return Some(EntityDirectionState::East);
    }

    None
}

// Direction texture updater
#[derive(Event)]
pub struct DirectionChangeEvent(pub Entity, pub EntityDirectionState);

// Важно создать систему, которая будет контролировать все сущности
/*
    Сущность перемещается, при изменение направления движения изменяется атрибут направления, когда атрибут направления изменён происходит event,
    который получает ссылку на entity и изменённое направление. Когда ивент создан, наверно, происходит выполнения функции, в котором ивент считывается
*/

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
                        Some(EntityDirectionState::South) => {
                            entity.0.direction = EntityDirectionState::South;
                            entity.1.index = EntityDirectionState::calculate_index(*index,  entity.0.direction.dir_index())
                        }
                        Some(EntityDirectionState::North) => {
                            entity.0.direction = EntityDirectionState::North;
                            entity.1.index = EntityDirectionState::calculate_index(*index,  entity.0.direction.dir_index())
                        }
                        Some(EntityDirectionState::East) => {
                            entity.0.direction = EntityDirectionState::East;
                            entity.1.index = EntityDirectionState::calculate_index(*index,  entity.0.direction.dir_index())
                        }
                        Some(EntityDirectionState::West) => {
                            entity.0.direction = EntityDirectionState::West;
                            entity.1.index = EntityDirectionState::calculate_index(*index,  entity.0.direction.dir_index())
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
                                    Some(EntityDirectionState::South) => {
                                        if entity_b.1.direction != EntityDirectionState::North {
                                            entity_h.0.direction = EntityDirectionState::South;
                                            entity_h.1.index = EntityDirectionState::calculate_index(*index,  entity_h.0.direction.dir_index())
                                        }
                                    }
                                    Some(EntityDirectionState::North) => {
                                        if entity_b.1.direction != EntityDirectionState::South {
                                            entity_h.0.direction = EntityDirectionState::North;
                                            entity_h.1.index = EntityDirectionState::calculate_index(*index,  entity_h.0.direction.dir_index())
                                        }
                                    }
                                    Some(EntityDirectionState::East) => {
                                        if entity_b.1.direction != EntityDirectionState::West {
                                            entity_h.0.direction = EntityDirectionState::East;
                                            entity_h.1.index = EntityDirectionState::calculate_index(*index,  entity_h.0.direction.dir_index())
                                        }
                                        
                                    }
                                    Some(EntityDirectionState::West) => {
                                        if entity_b.1.direction != EntityDirectionState::East {
                                            entity_h.0.direction = EntityDirectionState::West;
                                            entity_h.1.index = EntityDirectionState::calculate_index(*index,  entity_h.0.direction.dir_index())
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