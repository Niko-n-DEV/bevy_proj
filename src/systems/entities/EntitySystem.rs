use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::core::{
    graphic::Atlas::{DirectionAtlas, TestTextureAtlas}, player::PlayerEntity::PlayerEntity, Entity::{EntityBase, Health, Position, Speed, Velocity}, Movement::DirectionState
};

#[derive(Component)]
pub struct EnemySpawner {
    pub cooldown: f32,
    pub timer: f32,
}

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
            println!("{}", spawn_transform.translation);
            commands
                .spawn(SpriteSheetBundle {
                    texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: TestTextureAtlas::get_index("mob", &handle)
                },
                    ..default()
                })
                .insert(EntityBase {
                    speed: Speed(100. , 150. , 50.),
                    health: Health(1.0),
                    position: Position(spawn_transform.translation),
                    direction: DirectionState::South,
                    velocity: Velocity(Vec3::ZERO),
                    movable: true
                });
        }
    }
}

// ==================================================
// -= Test =-
// Обновление врагов
// В данном случае движение их в сторону позиции игрока
// ==================================================
pub fn update_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&EntityBase, &mut Transform, Entity), Without<PlayerEntity>>,
    player_query: Query<(&PlayerEntity, &mut Transform), Without<EntityBase>>,
    mut commands: Commands,
) {
    if let Ok((_player_movement, player_transform)) = player_query.get_single() {
        for (enemy, mut transform, entity) in enemy_query.iter_mut() {
            let moving = Vec3::normalize(player_transform.translation - transform.translation)
                * enemy.speed.0
                * time.delta_seconds();
            transform.translation += moving;

            if enemy.health.0 <= 0. {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub struct EntitySystem;

impl Plugin for EntitySystem {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DirectionEvent>()
            .add_systems(Update, handle_direction_changed_events)
        ;
    }
}

// Dir

#[derive(Event)]
pub struct DirectionEvent {
    entity: EntityBase,
    new_direction: DirectionState,
}

// Важно создать систему, которая будет контролировать все сущности
/*
    Сущность перемещается, при изменение направления движения изменяется атрибут направления, когда атрибут направления изменён происходит event,
    который получает ссылку на entity и изменённое направление. Когда ивент создан, наверно, происходит выполнения функции, в котором ивент считывается
*/

// скорее будет работать по ивенту, по типу if direction_entity_is_change -> изменение текстуры на другое направление
/// Обновляет текстуру моба в зависимости от его направления
fn handle_direction_changed_events(
    //mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut query: Query<(&mut DirectionAtlas, &DirectionState)>,
    // direction_changed_events: Res<Events<DirectionChangedEvent>>,
    mut event: EventReader<DirectionEvent>
) {
    // for event in direction_changed_events.iter() {
    //     if let Ok((mut sprite, direction)) = query.get_mut(event.entity) {
    //         // Здесь вы можете изменить текстуру в соответствии с новым направлением
    //         match direction.0 {
    //             DirectionState::Up => {
    //                 // Изменить текстуру на текстуру для движения вверх
    //             }
    //             DirectionState::Down => {
    //                 // Изменить текстуру на текстуру для движения вниз
    //             }
    //             DirectionState::Left => {
    //                 // Изменить текстуру на текстуру для движения влево
    //             }
    //             DirectionState::Right => {
    //                 // Изменить текстуру на текстуру для движения вправо
    //             }
    //         }
    //     }
    // }
}