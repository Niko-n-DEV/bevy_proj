use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::Entity::{Health, Position, Speed, Velocity};
#[allow(unused_imports)]
use crate::core::{
    entities::EntitySystem::{update_enemies, update_spawning, DirectionChangeEvent, EnemySpawner},
    items::Weapon::*,
    resource::graphic::Atlas::{DirectionAtlas, TestTextureAtlas},
    AppState,
    Entity::EntityBase,
    Input::OffsetedCursorPosition,
    Input::{cursor_track, CursorPosition},
    Missile::*,
    Movement::DirectionState,
};

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct PlayerEntity; // Rename to User

//
// Всё это под вырез, т.к. будет переносится в Entity System с взаимодействием с Entity, а player будет как свойство для entity
//
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Установка игрока при переходе в стостояние "игра"
            //.add_systems(OnEnter(AppState::Game), Self::spawn_player)
            // Регистрация типа "PlayerEntity" для индексации параметров в Инспекторе
            .register_type::<PlayerEntity>()
            // Использование данных о позиции курсора из CursorPosition
            .init_resource::<CursorPosition>()
            // Инициализация чего-то (я сам до конца не понял)
            .insert_resource(OffsetedCursorPosition { x: 0., y: 0. })
            // Передвижение игрока
            .add_systems(
                Update,
                Self::player_movement.run_if(in_state(AppState::Game)),
            )
            // Обновление информации о позиции курсора
            .add_systems(PreUpdate, cursor_track.run_if(in_state(AppState::Game)))
            // [Test] Обновление системы управления оружием
            .add_systems(Update, gun_controls.run_if(in_state(AppState::Game)))
            // [Test] Соединение оружия и игрока
            .add_systems(Update, attach_objects.run_if(in_state(AppState::Game)))
            // [Test] Обновление системы просчёта пуль и попадений
            .add_systems(
                Update,
                (
                    update_bullets.run_if(in_state(AppState::Game)),
                    update_bullet_hits.run_if(in_state(AppState::Game)),
                ),
            )
            // [Test] Обновление системы просчёта врагов и их спавна
            .add_systems(
                Update,
                (
                    update_enemies.run_if(in_state(AppState::Game)),
                    update_spawning.run_if(in_state(AppState::Game)),
                ),
            )
            // Инициализация "удаления" игрока при переходе из состояния "игра"
            .add_systems(OnExit(AppState::Game), Self::despawn_player);
    }
}

impl PlayerPlugin {
    /// "Удаление" игрока
    fn despawn_player(
        mut commands: Commands,
        player: Query<Entity, With<PlayerEntity>>,
        gun: Query<Entity, With<GunController>>,
    ) {
        if let Ok(player) = player.get_single() {
            commands.entity(player).despawn_recursive()
        }
        if let Ok(gun) = gun.get_single() {
            commands.entity(gun).despawn_recursive()
        }
    }

    // Query<(&mut Transform, &mut EntityBase), With<PlayerEntity>>
    /// Передвижение игрока
    fn player_movement(
        mut entity_query: Query<(&mut Transform, &mut EntityBase), With<PlayerEntity>>,
        player: Query<Entity, With<PlayerEntity>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        _mouse_input: Res<OffsetedCursorPosition>,
        mut change_dir_event: EventWriter<DirectionChangeEvent>,
        time: Res<Time>,
    ) {
        if entity_query.is_empty() {
            return;
        }

        let entity = player.single();
        for (mut transform, mut player) in &mut entity_query {
            if player.movable {
                let mut direction = Vec3::ZERO;

                let mut dir_state_temp = DirectionState::default();
                let mut dir_state = DirectionState::default();

                let mut speed_var: f32 = player.speed.0;

                if keyboard_input.pressed(KeyCode::ShiftLeft) {
                    speed_var = player.speed.1;
                }

                if keyboard_input.pressed(KeyCode::KeyW) {
                    dir_state = DirectionState::North;
                    dir_state_temp = DirectionState::North;
                    direction.y += 1.0;
                    //change_dir_event.send(DirectionChangeEvent(entity, DirectionState::North));
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    dir_state = DirectionState::South;
                    dir_state_temp = DirectionState::South;
                    direction.y -= 1.0;
                    //change_dir_event.send(DirectionChangeEvent(entity, DirectionState::South));
                }
                if keyboard_input.pressed(KeyCode::KeyA) {
                    dir_state = DirectionState::West;
                    dir_state_temp = DirectionState::West;
                    direction.x -= 1.0;
                    //change_dir_event.send(DirectionChangeEvent(entity, DirectionState::West));
                }
                if keyboard_input.pressed(KeyCode::KeyD) {
                    dir_state = DirectionState::East;
                    dir_state_temp = DirectionState::East;
                    direction.x += 1.0;
                    //change_dir_event.send(DirectionChangeEvent(entity, DirectionState::East));
                }

                if player.direction != dir_state_temp {
                    change_dir_event.send(DirectionChangeEvent(entity, dir_state_temp));

                    player.direction = dir_state;
                }

                if direction != Vec3::ZERO {
                    let new_pos = transform.translation
                        + time.delta_seconds() * speed_var * direction.normalize();
                    transform.translation = new_pos;
                    player.position = Position(new_pos);
                } else {
                    transform.translation = player.position.0
                }
            }
        }
    }
}

// По идее это внутренний компонент инвентаря игрока, но пока что он не реализован
#[derive(Component, InspectorOptions)]
pub struct Inventory {}

#[derive(Component)]
pub struct PlayerAttach {
    pub offset: Vec2,
}

// Просто хранение информации о том, где игрок.
// Используется для применение этих данных объектам, для их привязки к игроку.

pub fn attach_objects(
    player_query: Query<(&PlayerEntity, &mut Transform), Without<PlayerAttach>>,
    mut objects_query: Query<(&PlayerAttach, &mut Transform), Without<PlayerEntity>>,
) {
    if let Ok((_, player_transform)) = player_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            transform.translation =
                player_transform.translation + Vec3::new(attach.offset.x, attach.offset.y, 1.);
        }
    }
}
