use bevy::prelude::*;
//use bevy_rapier2d::prelude::*;

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
    Weapon::*,
    resource::graphic::Atlas::{DirectionAtlas, TestTextureAtlas},
    AppState,
    Entity::{EntityBase, Position},
    UserSystem::CursorPosition,
    Missile::*,
    Movement::DirectionState,
    world::World::WorldSystem,
    UserSystem::User,
    ItemType::{
        Pickupable,
        ItemType
    },
    world::chunk::Chunk::Chunk,
    ContainerSystem::{
        ItemPickUpEvent,
        ItemDropEvent,
        Container,
        Inventory,
        ItemTypeEx
    }
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
        mut entity_query:       Query<(&mut Transform, &mut EntityBase, Entity), With<User>>,
        mut move_event:         EventWriter<MovementEntity>,
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
                    move_event.send(MovementEntity(entity, direction, speed_var));
                }
            }
        }
    }

    fn player_pickup(
        // mut commands:           Commands,
        mut chunk_res:          ResMut<Chunk>,
        mut user:               Query<(&mut Inventory, &EntityBase, &Transform), With<User>>,
            keyboard_input:     Res<ButtonInput<KeyCode>>,
        //mut user_query:         Query<(&Transform, &EntityBase, &mut Container), With<User>>,
            pickupable_quety:   Query<(Entity, &Transform, &Pickupable, &Name), Without<User>>
    ) {
        if pickupable_quety.is_empty() || user.is_empty() {
            return;
        }

        //let (user_transform, user, mut container) = user_query.single_mut();
        let mut user = user.single_mut();

        if keyboard_input.just_pressed(KeyCode::Space) {
            for (entity, transform, pick, name) in pickupable_quety.iter() {
                if user.1.interaction_radius > Vec3::distance(transform.translation, user.2.translation) {
                    if user.0.add((entity, name.to_string(), pick.count)) {
                        chunk_res.remove_sub_object_ex(entity);
                        
                        // commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }

    // pub fn pick_up_items<I: ItemTypeEx>(
    //     mut cmd:            Commands,
    //     mut pickup_event:   EventReader<ItemPickUpEvent>,
    //     mut user:           Query<(&mut Inventory, &EntityBase, &Transform), With<User>>,
    //         items: Query<
    //             (Entity, &Transform, &I, &Pickupable, &Children),
    //             (
    //                 With<Transform>,
    //                 With<GlobalTransform>,
    //                 With<Visibility>,
    //             ),
    //         >,
    // ) {
    //     for event in pickup_event.read() {
    //         if let Ok((mut inv, user, transform)) = user.get_mut(event.picker) {

    //             // for (item_entity, _, item_type, pick, children) in
    //             //     items.iter().filter(|(_, pt, _, _)| **pt == *actor_pt)
    //             // {
    //             //     if equipment.add(item_entity, item_type) || inventory.add(item_entity) {
    //             //         for c in children.iter() {
    //             //             cmd.entity(*c).despawn_recursive();
    //             //         }
    //             //         cmd.entity(item_entity)
    //             //             .remove::<Transform>()
    //             //             .remove::<GlobalTransform>()
    //             //             .remove::<Visibility>();
    //             //     }
    //             // }

    //         }
    //     }
    // }
    
    // pub fn drop_item<I: ItemTypeEx>(
    //     mut cmd: Commands,
    //     mut drop_reader: EventReader<ItemDropEvent>,
    //     mut actors: Query<(&Vector2D, &mut Inventory, &mut Equipment<I>)>,
    // ) {
    //     for e in drop_reader.iter() {
    //         if let Ok((pt, mut inventory, mut equipment)) = actors.get_mut(e.droper) {
    //             if inventory.take(e.item) || equipment.take(e.item) {
    //                 cmd.entity(e.item).insert(*pt);
    //             }
    //         }
    //     }
    // }
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
