use bevy::prelude::*;

//use bevy_inspector_egui::prelude::ReflectInspectorOptions;
//use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    entities::EntitySystem::MovementEntity,
    // Weapon::*,
    AppState,
    Entity::{
        EntityBase,
        EntityHead,
    },
    stats::Stats,
    UserSystem::CursorPosition,
    world::World::WorldSystem,
    UserSystem::{
        CursorMode,
        UserControl,
        UserSubControl,
    },
    Item::{
        TakeItem,
        ItemSpawn,
    },
    // ItemType::ItemEntity,
    // world::chunk::Chunk::Chunk,
    ContainerSystem::{
        CursorContainer,
        // Inventory
    },
    interact::Damage::DamageObject
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Передвижение игрока
            .add_systems(Update, Self::player_movement.run_if(in_state(AppState::Game)))
            // Подбирание игроком предметов поддающиеся к подниманию
            .add_systems(Update, 
                (
                    Self::player_pickup,
                    Self::item_drop
                ).run_if(in_state(AppState::Game))
            )
            // Удар игроков по объекту
            .add_systems(Update, 
                (
                    Self::combat_toggle,
                    Self::player_atack
                ).run_if(in_state(AppState::Game))
            )
            // [Test] Обновление системы управления оружием
            // .add_systems(Update, gun_controls.run_if(in_state(AppState::Game)))
            // [Test] Соединение оружия и игрока
            .add_systems(PostUpdate, attach_objects.run_if(in_state(AppState::Game)))
            .add_systems(Update, Self::head_movement.run_if(in_state(AppState::Game)))
        ;
    }
}

impl PlayerPlugin {
    
    fn player_movement(
        mut entity_query:       Query<(&mut Transform, &mut EntityBase, Entity), With<UserControl>>,
        mut _move_event:         EventWriter<MovementEntity>,
            keyboard_input:     Res<ButtonInput<KeyCode>>
    ) {
        if entity_query.is_empty() {
            return;
        }

        for (mut _transform, player, entity) in &mut entity_query {
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
                    _move_event.send(MovementEntity(entity, direction, speed_var));
                }
            }
        }
    }

    fn head_movement(
        mut head:   Query<&mut EntityHead, With<UserSubControl>>,
            cursor: Res<CursorPosition>,
    ) {
        if head.is_empty() {
            return;
        }

        if let Ok(mut head) = head.get_single_mut() {
            if head.look_at != cursor.0 {
                head.look_at = cursor.0
            }
        }
    }

    fn combat_toggle(
        mut cursor_mode:    ResMut<CursorMode>,
            keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::KeyR) {
            if *cursor_mode != CursorMode::Atack {
                info!("Combat mode - enabled");
                *cursor_mode = CursorMode::Atack
            } else {
                info!("Combat mode - disable");
                *cursor_mode = CursorMode::None
            }
        }
    }

    fn player_atack(
            cursor_mode:    Res<CursorMode>,
        // mut chunk_res:      ResMut<Chunk>,
            cursor:         Res<CursorPosition>,
            mouse_input:    Res<ButtonInput<MouseButton>>,
            user:           Query<(&EntityBase ,&Transform, &Stats), With<UserControl>>,
        //    object:         Query<(Entity, &Transform), With<EntityObject>>,
        // entity: Query<(&mut EntityBase, &Transform), With<EntityBase>>,
        mut event:          EventWriter<DamageObject>
    ) {
        if user.is_empty() {
            return;
        }

        if *cursor_mode == CursorMode::Atack {
            if mouse_input.just_pressed(MouseButton::Left) {
                if let Ok(player) = user.get_single() {
                    if player.0.atack_radius > Vec3::distance(cursor.0.extend(0.5), player.1.translation) {
                        event.send(DamageObject(cursor.0.as_ivec2(), player.2.str as f32));
                    }
                }
            }
        }
    }

    fn player_pickup(
        // mut commands:           Commands,
        // mut chunk_res:          ResMut<Chunk>,
        user:                   Query<Entity, With<UserControl>>,
        mut event:              EventWriter<TakeItem>,
            keyboard_input:     Res<ButtonInput<KeyCode>>,
            // pickupable_quety:   Query<(Entity, &Transform, &ItemEntity), Without<UserControl>>
    ) {
        if user.is_empty() {
            return;
        }

        let user = user.single();

        if keyboard_input.just_pressed(KeyCode::KeyE) {
            // for (entity, transform, item) in pickupable_quety.iter() {
            //     if user.1.interaction_radius > Vec3::distance(transform.translation, user.2.translation) {
            //         if user.0.add(item.clone()) {
            //             chunk_res.remove_sub_object_ex(entity);
                        
            //             commands.entity(entity).despawn_recursive();
            //         }
            //     }
            // }
            event.send(TakeItem(user, None));
        }
    }

    fn item_drop(
        mut cursor_c:       ResMut<CursorContainer>,
        mut spawn_i:        EventWriter<ItemSpawn>,
            user:           Query<(&EntityBase, &Transform), With<UserControl>>,
            cursor:         Res<CursorPosition>,
            keyboard_input: Res<ButtonInput<KeyCode>>,
        //    mouse_input:    Res<ButtonInput<MouseButton>>,
    ) {
        if cursor_c.slot.is_none() && user.is_empty() {
            return;
        }

        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            let player = user.single();
            if player.0.interaction_radius > Vec3::distance(cursor.0.extend(0.5), player.1.translation) {
                if let Some(slot) = cursor_c.slot.take() {
                    spawn_i.send(ItemSpawn(slot.id_name, WorldSystem::get_currect_chunk_subtile(cursor.0.as_ivec2()), slot.count));
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
    player_query:       Query<(&UserControl, &mut Transform), Without<PlayerAttach>>,
    mut objects_query:  Query<(&PlayerAttach, &mut Transform), Without<UserControl>>,
) {
    if let Ok((_, player_transform)) = player_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            transform.translation =
                player_transform.translation + Vec3::new(attach.offset.x, attach.offset.y, 1.);
        }
    }
}
