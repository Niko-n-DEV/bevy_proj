use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

//use bevy_inspector_egui::prelude::ReflectInspectorOptions;
//use bevy_inspector_egui::InspectorOptions;

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
    UserSystem::CursorPosition,
    Missile::*,
    Movement::DirectionState,
    world::World::WorldSystem,
    UserSystem::User,
    items::ItemType::{
        Pickupable,
        ItemType
    },
    Container::Container
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Передвижение игрока
            .add_systems(Update, Self::player_movement.run_if(in_state(AppState::Game)))
            // Подбирание игроком предметов поддающиеся к подниманию
            .add_systems(Update, Self::player_pickup.run_if(in_state(AppState::Game)))
            // [Test] Обновление системы управления оружием
            .add_systems(Update, gun_controls.run_if(in_state(AppState::Game)))
            // [Test] Соединение оружия и игрока
            .add_systems(PostUpdate, attach_objects.run_if(in_state(AppState::Game)))
            ;
    }
}

impl PlayerPlugin {
    
    fn player_movement(
        mut entity_query:       Query<(&mut Transform, &mut EntityBase, &mut Velocity, Entity), With<User>>,
        mut move_event:         EventWriter<MovementEntity>,
            keyboard_input:     Res<ButtonInput<KeyCode>>
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

    fn player_pickup(
        mut commands:       Commands,
        keyboard_input:     Res<ButtonInput<KeyCode>>,
        mut user_query:     Query<(&Transform, &EntityBase, &mut Container), With<User>>,
        pickupable_quety:   Query<(Entity, &Transform, &Pickupable), Without<User>>
    ) {
        if pickupable_quety.is_empty() {
            return;
        }

        let (user_transform, user, mut container) = user_query.single_mut();

        if keyboard_input.just_pressed(KeyCode::Space) {
            for (entity, transform, pick) in pickupable_quety.iter() {
                if user.interaction_radius > Vec3::distance(transform.translation, user_transform.translation) {
                    if container.add_in_container(pick.item, pick.count) {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

#[allow(unused)]
#[derive(Component)]
pub struct ItemCursorPick;

#[derive(Component)]
pub struct PlayerAttach {
    pub offset: Vec2,
}

// Просто хранение информации о том, где игрок.
// Используется для применение этих данных объектам, для их привязки к игроку.

pub fn attach_objects(
    player_query:       Query<(&User, &mut Transform), Without<PlayerAttach>>,
    mut objects_query:  Query<(&PlayerAttach, &mut Transform), Without<User>>,
) {
    if let Ok((_, player_transform)) = player_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            transform.translation =
                player_transform.translation + Vec3::new(attach.offset.x, attach.offset.y, 1.);
        }
    }
}
