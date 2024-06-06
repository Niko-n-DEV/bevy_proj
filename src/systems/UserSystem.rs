#![allow(unused)]
use bevy::{
    prelude::*, 
    window::PrimaryWindow
};

// use bevy_rapier2d::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    resource::{
        // SpriteLayer,
        Registry::Registry,
        graphic::Atlas::{
            AtlasType,
            AtlasRes,
        }
    },
    AppState,
    world::{
        World::WorldSystem,
        chunk::Chunk::Chunk
    },
    Camera::UserCamera,
    Entity::{
        EntityBase,
        EntityHead,
        EntitySpawn,
    },
    Item::ItemSpawn,
    Object::ObjectSpawn,
    ContainerSystem::Inventory,
};

#[derive(Component, InspectorOptions, Reflect, Resource)]
#[reflect(Component, InspectorOptions)]
pub struct User {
    pub uid:            usize,
    pub user_name:      String,
    pub control_entity: Option<Entity>
}

#[derive(Component)]
pub struct UserControl {
    pub uid:            usize,
    pub user_name:      String,
}

#[derive(Component)]
pub struct UserSubControl {
    pub uid:            usize,
    pub user_name:      String,
}


impl Default for User {
    fn default() -> Self {
        Self {
            uid: 0,
            user_name: "Admin".to_string(),
            control_entity: None
        }
    }
}

impl User {
    pub fn user_is_controled_entity(
        user: &User
    ) -> bool {
        !user.control_entity.is_none()
    }

    pub fn control_entity_update(
        mut user:       ResMut<User>,
            control:    Query<Entity, Added<UserControl>>
        // user: &mut User,
        // entity: Entity
    ) {
        if control.is_empty() {
            return;
        }
        
        if let Ok(entity) = control.get_single() {
            user.control_entity = Some(entity);
        }
    }

    pub fn to_control(
        mut commands:       Commands,
            user:           Res<User>,
            cursor:         Res<CursorPosition>,
            entity_b:       Query<(Entity, &Transform), With<EntityBase>>,
            entity_h:       Query<(Entity, &EntityHead, &Transform), With<EntityHead>>,
            keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.pressed(KeyCode::ControlLeft) && keyboard_input.just_pressed(KeyCode::F5) {
            if user.control_entity.is_none() {
                for (entity_b, transform) in &entity_b {
                    if 8.0 > Vec3::distance(transform.translation, cursor.0.extend(0.5)) {
                        commands.entity(entity_b)
                            .insert(UserControl {
                                uid: user.uid,
                                user_name: user.user_name.clone()
                            })
                            .insert(Inventory::with_capacity(12));

                        for (entity_h, head, transform) in &entity_h {
                            if head.parent == entity_b {
                                commands.entity(entity_h)
                                    .insert(UserSubControl {
                                        uid: user.uid,
                                        user_name: user.user_name.clone()
                                    });
                            }
                        }

                        return;
                    }
                }
            }
        }
    }

    pub fn remove_control(
        mut commands:   Commands,
        mut user:       ResMut<User>,
            query:      Query<Entity, With<UserControl>>,
            query_h:    Query<Entity, With<UserSubControl>>
    ) {
        if query.is_empty() {
            return;
        }

        if let Ok(entity) = query.get_single() {
            commands.entity(entity).remove::<UserControl>();

            if !query_h.is_empty() {
                if let Ok(entity) = query_h.get_single() {
                    commands.entity(entity).remove::<UserSubControl>();
                }
            }

            user.control_entity = None;
        }
    }
}



// Поместить сюда UserPlugin
pub struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app
            // Регистрация типа "User" для индексации параметров в Инспекторе
            .register_type::<User>()
            .insert_resource(User { ..default() })
            // Использование данных о позиции курсора из CursorPosition
            .init_resource::<CursorPosition>()
            .init_resource::<CursorProcentPos>()
            .init_resource::<CursorPlacer>()
            .init_resource::<CursorMode>()
            // Обновление информации о позиции курсора
            .add_systems(PreUpdate, cursor_track.run_if(in_state(AppState::Game)))
            .add_systems(Update, 
                (
                    placer,
                    delete_object,
                    attach_to_cursor,
                    create_text_placer,
                    attach_to_cursor
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(Update,
                (
                    User::control_entity_update,
                    User::to_control
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(OnExit(AppState::Game), User::remove_control)
        ;
    }
}

// ==============================
// Cursor Position
// ==============================

/// Позиция курсора на на окне
#[derive(Resource, Default)]
pub struct CursorPosition(pub Vec2);

/// Процентная позиция курсора с учётом размера окна
#[derive(Resource, Default)]
pub struct CursorProcentPos(pub Vec2);

pub fn cursor_track(
    mut cursor_pos:     ResMut<CursorPosition>,
    mut cursor_procent: ResMut<CursorProcentPos>,
        window:         Query<&Window, With<PrimaryWindow>>,
        camera:         Query<(&Camera, &GlobalTransform), With<UserCamera>>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_pos.0 = world_position;
    }
    
    if let Some(position) = window.cursor_position() {
        cursor_procent.0 = Vec2::new(
            (position.x / window.resolution.physical_width() as f32) * 100.0, 
            (position.y / window.resolution.physical_height() as f32) * 100.0
        );
    }
}



// ==============================
// CursorMode
// ==============================

#[derive(Default, Resource, PartialEq)]
pub enum CursorMode {
    #[default]
    None,
    Placer,
    Build,
    Atack,
}

// ==============================
// Placer
// ==============================

/// Компонент отвечающий за хранение информации о распалогаемом объекте на месте курсора
#[derive(Resource, Default)]
pub struct CursorPlacer {
    pub placer: Option<(String, String)>,
    pub entity: Option<Entity>
}

fn placer(
    mut cursor_mode:    ResMut<CursorMode>,
    mut placer:         ResMut<CursorPlacer>,
    mut obj_event:      EventWriter<ObjectSpawn>,
    mut item_event:     EventWriter<ItemSpawn>,
    mut entity_event:   EventWriter<EntitySpawn>,
        cursor:         Res<CursorPosition>,
        mouse_input:    Res<ButtonInput<MouseButton>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        registry:       Res<Registry>,
) {
    if *cursor_mode == CursorMode::Placer {
        if mouse_input.just_pressed(MouseButton::Left) {
            if let Some(match_type) = placer.placer.clone() {
                match match_type.0.as_str() {
                    "item" => {
                        item_event.send(ItemSpawn(match_type.1, WorldSystem::get_currect_chunk_subtile(cursor.0.as_ivec2()), 1));
                    },
                    "object" => {
                        obj_event.send(ObjectSpawn(match_type.1, cursor.0.as_ivec2()));
                    },
                    "entity" => {
                        entity_event.send(EntitySpawn(match_type.1, cursor.0));
                    },
                    _ => warn!("Неверный указанный тип!")
                }
            }
        }

        if keyboard_input.just_pressed(KeyCode::Escape) {
            placer.placer = None;
            *cursor_mode = CursorMode::None;
        }
    }
}

fn create_text_placer(
    mut commands:   Commands,
    mut placer:     ResMut<CursorPlacer>,
        cursor_p:   Res<CursorProcentPos>,
) {
    if placer.is_changed() {
        if let Some(entity) = placer.entity {
            commands.entity(entity).despawn_recursive();
            placer.entity = None
        }

        if !placer.placer.is_none() {
            if let Some(text) = placer.placer.clone() {
                placer.entity = Some(commands.spawn((
                    TextBundle {
                        style: Style {
                            position_type:  PositionType::Absolute,
                            left:           Val::Percent(cursor_p.0.x + 1.0),
                            top:            Val::Percent(cursor_p.0.y - 1.0),
                            height:         Val::Percent(5.0),
                            width:          Val::Percent(3.0),
                            ..default()
                        },
                        text: Text {
                            sections: vec![TextSection::new(
                                format!("{}", text.1),
                                TextStyle {
                                    font_size: 11.0,
                                    ..default()
                                },
                            )],
                            ..default()
                        },
                        ..default()
                    },
                    InfoTextPlace
                )).id());
            }
        }
    }
}

#[derive(Component)]
pub struct InfoTextPlace;

fn attach_to_cursor(
        cursor: Res<CursorProcentPos>,
        placer: ResMut<CursorPlacer>,
    mut text:   Query<&mut Style, With<InfoTextPlace>>
) {
    if text.is_empty() {
        return;
    }

    if !placer.placer.is_none() {
        if let Ok(mut text) = text.get_single_mut() {
            // text.translation = Vec3::new(cursor.0.x + 8.0, cursor.0.y + 8.0, 0.0);
            text.left    = Val::Percent(cursor.0.x + 1.0);
            text.top     = Val::Percent(cursor.0.y - 1.0);
        }
    }
}

// ==============================
// ?
// ==============================

// Скорее всего определение может ли объект быть выделенным
#[derive(Component)]
pub struct Selectable {
    pub is_selected: bool
}

// ==============================
// SelectBox
// ==============================

#[derive(Resource)]
pub struct SelectBox {
    pub is_active:          bool,
    pub start_position:     Vec2,
    pub current_position:   Vec2,
}

// pub fn _update_select_box(
//     mut query: Query<(
//         &Camera2d,
//         &Camera,
//         &GlobalTransform
//     ), With<UserCamera>>,
//     mut select_box:     ResMut<SelectBox>,
//         mouse_input:    Res<ButtonInput<MouseButton>>,
//         windows:        Query<&Window, With<PrimaryWindow>>
// ) {
//     let (_, camera, global_transform) = query.single_mut();

//     // let window = if let RenderTarget::Window(id) = camera.target {
//     //     windows.get(id).unwrap()
//     // } else {
//     //     windows.get_primary().unwrap()
//     // };

//     let window = windows.single();

//     if let Some(win_pos) = window.cursor_position() {

//         let window_size     = Vec2::new(window.width() as f32, window.height() as f32);
//         let ndc             = (win_pos / window_size) * 2.0 - Vec2::ONE;
//         let ndc_to_world    = global_transform.compute_matrix() * camera.projection_matrix().inverse();
//         let world_pos       = ndc_to_world.project_point3(ndc.extend(-1.0));
//         let world_pos: Vec2       = world_pos.truncate();

//         if mouse_input.just_pressed(MouseButton::Left) {
//             select_box.is_active = true;
//             select_box.start_position = world_pos;
//             select_box.current_position = world_pos;
//         }

//         if mouse_input.pressed(MouseButton::Left) {
//             select_box.is_active = true;
//             select_box.current_position = world_pos;
//         }

//         if mouse_input.just_released(MouseButton::Left) {
//             select_box.is_active = false;
//         }
//     }
// }

pub fn def_select_box(
    mut select_box:     ResMut<SelectBox>,
        cursor:         Res<CursorPosition>,
        mouse_input:    Res<ButtonInput<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        select_box.is_active        = true;
        select_box.start_position   = cursor.0;
        select_box.current_position = cursor.0;
    }

    if mouse_input.pressed(MouseButton::Left) {
        select_box.is_active        = true;
        select_box.current_position = cursor.0;
    }

    if mouse_input.just_released(MouseButton::Left) {
        select_box.is_active = false;
    }
}

pub fn update_select_box(
    mut select_box:     ResMut<SelectBox>,
) {
    if select_box.is_active {

    }
}

// ==============================
// Test
// ==============================

fn delete_object(
    mut commands:       Commands,
        cursor:         Res<CursorPosition>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chunk_res:      ResMut<Chunk>
) {
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        let tiled_pos = WorldSystem::get_currect_chunk_tile(cursor.0.as_ivec2());
        if let Some(entity) = chunk_res.remove_object(&tiled_pos) {
            commands.entity(entity).despawn_recursive();
        }
    }
}