use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::dynamics::Velocity;

#[allow(unused_imports)]
use crate::core::{
    entities::EntitySystem::{
        update_enemies, 
        update_spawning, 
        DirectionChangeEvent, 
        EnemySpawner,
        MovementEntity
    },
    items::Weapon::*,
    resource::graphic::Atlas::{DirectionAtlas, TestTextureAtlas},
    AppState,
    Entity::{EntityBase, Position},
    Input::OffsetedCursorPosition,
    Input::{cursor_track, CursorPosition},
    Missile::*,
    Movement::DirectionState,
};

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct User {
    pub uid: usize,
    pub user_name: String
}

impl Default for User {
    fn default() -> Self {
        Self {
            uid: 0,
            user_name: "Niko_n".to_string()
        }
    }
}

//
// Всё это под вырез, т.к. будет переносится в Entity System с взаимодействием с Entity, а player будет как свойство для entity
//
pub struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app
            // Установка игрока при переходе в стостояние "игра"
            //.add_systems(OnEnter(AppState::Game), Self::spawn_player)
            // Регистрация типа "PlayerEntity" для индексации параметров в Инспекторе
            .register_type::<User>()
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
            .add_systems(PostUpdate, attach_objects.run_if(in_state(AppState::Game)))
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
            ;
    }
}

impl UserPlugin {

    /// Передвижение игрока
    fn player_movement(
        mut entity_query: Query<(&mut Transform, &mut EntityBase, &mut Velocity, Entity), With<User>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        _mouse_input: Res<OffsetedCursorPosition>,
        mut move_event: EventWriter<MovementEntity>
    ) {
        if entity_query.is_empty() {
            return;
        }

        for (mut _transform, player, mut _vel, entity) in &mut entity_query {
            if player.movable {
                let mut direction = Vec3::ZERO;

                let mut speed_var: f32 = player.speed.0;

                if keyboard_input.pressed(KeyCode::ShiftLeft) {
                    speed_var = player.speed.1;
                }

                if keyboard_input.pressed(KeyCode::KeyW) {
                    direction.y += 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    direction.y -= 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyA) {
                    direction.x -= 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyD) {
                    direction.x += 1.0;
                }

                if direction != Vec3::ZERO {
                    move_event.send(MovementEntity(entity, direction.normalize(), speed_var));
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
    player_query: Query<(&User, &mut Transform), Without<PlayerAttach>>,
    mut objects_query: Query<(&PlayerAttach, &mut Transform), Without<User>>,
) {
    if let Ok((_, player_transform)) = player_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            transform.translation =
                player_transform.translation + Vec3::new(attach.offset.x, attach.offset.y, 1.);
        }
    }
}
