use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{
    core::{
        player::PlayerEntity::User,
        resource::graphic::Atlas::{DirectionAtlas, TestTextureAtlas},
        Entity::{
            EntityBase,
            Health,
            Position,
            //Speed,
            Velocity,
            EntityType,
            HumonoidType,
            EntityNeutrality
        },
        Movement::DirectionState,
    },
    AppState,
};

// ==============================
// TEST
// Enemy
// ==============================

#[derive(Component)]
pub struct EnemySpawner {
    pub cooldown: f32,
    pub timer: f32,
}

#[derive(Component)]
pub struct Enemy;

pub fn update_spawning(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut spawner_query: Query<&mut EnemySpawner>,
    time: Res<Time>,
    //asset_server: Res<AssetServer>,
    handle: Res<TestTextureAtlas>,
    mut commands: Commands,
) {
    for mut spawner in spawner_query.iter_mut() {
        spawner.timer -= time.delta_seconds();
        if spawner.timer <= 0. {
            let Ok(primary) = primary_query.get_single() else {
                return;
            };

            spawner.timer = spawner.cooldown;

            let mut spawn_transform = Transform::from_scale(Vec3::splat(5.));

            let mut rng = rand::thread_rng();

            if rng.gen_range(0..2) == 1 {
                if rng.gen_range(0..2) == 1 {
                    spawn_transform.translation = Vec3::new(
                        primary.width() / 2.,
                        rng.gen_range(-primary.height() / 2.0..primary.height() / 2.0),
                        0.,
                    );
                } else {
                    spawn_transform.translation = Vec3::new(
                        -primary.width() / 2.,
                        rng.gen_range(-primary.height() / 2.0..primary.height() / 2.0),
                        0.,
                    );
                }
            } else {
                if rng.gen_range(0..2) == 1 {
                    spawn_transform.translation = Vec3::new(
                        rng.gen_range(-primary.width() / 2.0..primary.width() / 2.0),
                        primary.height() / 2.,
                        0.,
                    );
                } else {
                    spawn_transform.translation = Vec3::new(
                        rng.gen_range(-primary.width() / 2.0..primary.width() / 2.0),
                        -primary.height() / 2.,
                        0.,
                    );
                }
            }

            commands
                .spawn(SpriteSheetBundle {
                    texture: handle.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: TestTextureAtlas::get_index("mob", &handle),
                    },
                    transform: spawn_transform,
                    ..default()
                })
                .insert(EntityBase {
                    health: Health(2.0),
                    direction: DirectionState::South,
                    velocity: Velocity(Vec3::ZERO),
                    movable: true,
                    ..default()
                })
                .insert(Enemy)
                .insert((
                    EntityType::Humonoid(HumonoidType::Human),
                    EntityNeutrality::Hostile,
                ));
        }
    }
}

// ==================================================
// -= Test =-
// Обновление врагов
// В данном случае движение их в сторону позиции игрока
// ==================================================
pub fn update_enemies(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, &EntityBase, Entity), With<Enemy>>,
    player_query: Query<(&Transform, &User), Without<Enemy>>,
    mut move_event: EventWriter<Movement>
) {
    if enemy_query.is_empty() || player_query.is_empty() {
        return;
    }

    if let Ok((player_transform, _player)) = player_query.get_single() {
        for (transform, enemy, entity) in enemy_query.iter_mut() {
            let direction = Vec3::normalize(player_transform.translation - transform.translation);

            move_event.send(Movement(entity, direction.normalize(), enemy.speed.0));

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
            .add_event::<DirectionChangeEvent>()
            .add_event::<Movement>()
            .add_systems(
                Update,
                (
                    handle_direction_changed_events.run_if(in_state(AppState::Game)),
                    handle_move.run_if(in_state(AppState::Game))
                )
            )
            .add_systems(OnExit(AppState::Game), delete_enemy_spawner)
        ;
    }
}

// Перенести в world, т.к. этот компонет спавнера
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
pub struct Movement(pub Entity, pub Vec3, pub f32);

fn handle_move(
    mut query: Query<(
        &mut EntityBase, 
        &mut Transform
    )>,
    mut event: EventReader<Movement>,
    mut dir_event: EventWriter<DirectionChangeEvent>,
    time: Res<Time>
) {
    if event.is_empty() {
        return;
    }

    for event in event.read() {
        if let Ok((mut entity_base, mut transform)) = query.get_mut(event.0) {
            if event.1 != Vec3::ZERO {
                dir_event.send(DirectionChangeEvent(event.0, determine_direction(event.1)));
                transform.translation = transform.translation + time.delta_seconds() * event.2 * event.1;
                entity_base.position = Position(transform.translation);
            } else {
                transform.translation = entity_base.position.0
            }
        }
    }
}

/// Функция для определения направления на основе нормализованного вектора
fn determine_direction(vector: Vec3) -> DirectionState {
    if vector == Vec3::ZERO {
        return DirectionState::None;
    }

    let angle = vector.y.atan2(vector.x).to_degrees().rem_euclid(360.0);

    if angle > 45.0 && angle <= 135.0 {
        return DirectionState::North;
    } else if angle > 135.0 && angle <= 225.0 {
        return DirectionState::West;
    } else if angle > 225.0 && angle <= 315.0 {
        return DirectionState::South;
    } else {
        return DirectionState::East;
    }
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
fn handle_direction_changed_events(
    mut _query: Query<(
        &mut EntityBase, 
        &mut TextureAtlas,
        &EntityType
    )>,
    _handle_dir: Res<DirectionAtlas>,
    mut event: EventReader<DirectionChangeEvent>,
) {
    if event.is_empty() {
        return;
    }

    let index_atlas = DirectionAtlas::get_index("human", &_handle_dir);
    for event in event.read() {
        if let Ok((mut _entity_base, mut atlas, _entity_type)) = _query.get_mut(event.0) {
            match event.1 {
                DirectionState::North => {
                    atlas.index = index_atlas + 1;
                    // info!("Hi - North")
                }
                DirectionState::South => {
                    atlas.index = index_atlas;
                    // info!("Hi - South")
                }
                DirectionState::West => {
                    atlas.index = index_atlas + 3;
                    // info!("Hi - West")
                }
                DirectionState::East => {
                    atlas.index = index_atlas + 2;
                    // info!("Hi - East")
                }
                DirectionState::None => {}
            }
        }
    }
}
