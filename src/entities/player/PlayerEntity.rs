use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[allow(unused_imports)]
use crate::core::{
    items::Weapon::*, 
    AppState, 
    Input::OffsetedCursorPosition, 
    Bullet::*, 
    Entity::{update_enemies, EntityBase}, 
    entities::EntitySys::{update_spawning, EnemySpawner}, 
    Input::{CursorPosition, cursor_track},
    graphic::Atlas::TestTextureAtlas
};

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct PlayerEntity {
    pub speed: f32,
    pub sprint: f32,
    pub position: Vec3,
    pub direction: DirectionState,
    pub velocity: Vec3,
    pub movable: bool,
}

impl Default for PlayerEntity {
    fn default() -> Self {
        Self {
            speed: 50.0,
            sprint: 125.0,
            position: Vec3::new(0.0, 0.0, 0.0),
            direction: DirectionState::South,
            velocity: Vec3::ZERO,
            movable: true,
        }
    }
}

impl PlayerEntity {}

pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app
            // Установка игрока при переходе в стостояние "игра"
            .add_systems(OnEnter(AppState::Game), Self::spawn_player)
            // Регистрация типа "PlayerEntity" для индексации параметров в Инспекторе
            .register_type::<PlayerEntity>()
            // Использование данных о позиции курсора из CursorPosition
            .init_resource::<CursorPosition>()
            // Инициализация чего-то (я сам до конца не понял)
            .insert_resource(OffsetedCursorPosition {x: 0., y: 0.})
            // Передвижение игрока
            .add_systems(Update, Self::player_movement.run_if(in_state(AppState::Game)))
            // Обновление информации о позиции курсора
            .add_systems(PreUpdate, cursor_track.run_if(in_state(AppState::Game)))
            // [Test] Обновление системы управления оружием
            .add_systems(Update, gun_controls.run_if(in_state(AppState::Game)))
            // [Test] Соединение оружия и игрока
            .add_systems(Update, attach_objects.run_if(in_state(AppState::Game)))
            // [Test] Обновление системы просчёта пуль и попадений
            .add_systems(Update, 
                (
                            update_bullets.run_if(in_state(AppState::Game)), 
                            update_bullet_hits.run_if(in_state(AppState::Game))
                        )
                    )
            // [Test] Обновление системы просчёта врагов и их спавна 
            .add_systems(Update, 
                (
                            update_enemies.run_if(in_state(AppState::Game)), 
                            update_spawning.run_if(in_state(AppState::Game))
                        )
                    )
            // Инициализация "удаления" игрока при переходе из состояния "игра"
            .add_systems(OnExit(AppState::Game), Self::despawn_player);
    }
}

impl Player {
    fn spawn_player(
        mut commands: Commands, 
        asset_server: Res<AssetServer>,
        handle: Res<TestTextureAtlas>
    ) {
        // Спавн спрайта, являющийся игроком
        commands.spawn((
            SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 3
                },
                ..default()
            },
            PlayerEntity::default(),
            Name::new("Player"),
            ));
        

        // Спавн оружия и соединение с игроком
        commands.spawn(SpriteSheetBundle {
            texture: handle.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: TestTextureAtlas::get_index("gun", &handle)
            },
            transform: Transform {
                translation: Vec3::splat(0.),
                ..default()
            },
            ..default()
        }).insert(PlayerAttach{ offset: Vec2::new(0.,0.)}).insert(GunController{ shoot_cooldown: 0.3, shoot_timer: 0. }); // 0.3

        // не переходить часто с главного меню в игру и на оборот, дублируются!
        //commands.spawn(TransformBundle { ..default() } ).insert(EnemySpawner{ cooldown: 1., timer: 1. });
    }

    /// "Удаление" игрока
    fn despawn_player(
        mut commands: Commands,
        player: Query<Entity, With<PlayerEntity>>,
    ) {
        if let Ok(player) = player.get_single() {
            commands.entity(player).despawn_recursive()
        }
    }

    /// Передвижение игрока
    fn player_movement(
        mut player_entity: Query<(&mut Transform, &mut PlayerEntity), With<PlayerEntity>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        _mouse_input: Res<OffsetedCursorPosition>,
        time: Res<Time>,
    ) {
        if player_entity.is_empty() {
            return;
        }

        for (mut transform, mut player) in &mut player_entity {
            if player.movable {
                let mut direction = Vec3::ZERO;
                let mut speed_var: f32 = player.speed;

                if keyboard_input.pressed(KeyCode::ShiftLeft) {
                    speed_var = player.sprint;
                }
                
                if keyboard_input.pressed(KeyCode::KeyW) {
                    player.direction = DirectionState::North;
                    direction.y += 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    player.direction = DirectionState::South;
                    direction.y -= 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyA) {
                    player.direction = DirectionState::West;
                    direction.x -= 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyD) {
                    player.direction = DirectionState::East;
                    direction.x += 1.0;
                }

                if direction != Vec3::ZERO {
                    let new_pos = transform.translation + time.delta_seconds() * speed_var * direction.normalize();
                    transform.translation = new_pos;
                    player.position = new_pos;
                } else {
                    let pos = player.position;
                    transform.translation = pos
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
