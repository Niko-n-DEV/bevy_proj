#![allow(unused)]
use bevy::{
    prelude::*, 
    window::PrimaryWindow
};

use bevy_rapier2d::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    resource::{
        SpriteLayer,
        Registry::Registry,
        graphic::Atlas::{
            TestTextureAtlas,
            AtlasRes
        }
    },
    AppState,
    world::World::WorldSystem,
    world::chunk::Chunk::Chunk,
    Camera::UserCamera,
    //Object::EntityObject,
    Entity::{
        EntityBase,
        EntityHead,
        EntitySpawn,
    },
    Item::ItemSpawn,
    Object::ObjectSpawn,
    ContainerSystem::Inventory,
    // ItemType::{
    //     Pickupable,
    //     ItemType,
    //     Item
    // }
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
            control:    Query<Entity, Added<UserControl>>,
        // user: &mut User,
        // entity: Entity
    ) {
        if control.is_empty() {
            return;
        }
        
        if let Ok(entity) = control.get_single() {
            user.control_entity = Some(entity);
            println!("place control entity")
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
        if keyboard_input.just_pressed(KeyCode::F5) {
            if user.control_entity.is_none() {
                for (entity_b, transform) in &entity_b {
                    if 16.0 > Vec3::distance(transform.translation, cursor.0.extend(0.5)) {
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
            .init_resource::<CursorPlacer>()
            .insert_resource(Selector { selector_entity: None, select_entity: None })
            // Обновление информации о позиции курсора
            .add_systems(PreUpdate, cursor_track.run_if(in_state(AppState::Game)))
            .add_systems(Update, 
                (
                    placer,
                    delete_object,
                    select_object,
                    selector_update,
                    selector_remove,
                    attach_to_select,
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
        ;
    }
}

// ==============================
// Cursor Position
// ==============================

/// Позиция курсора на глобальной координатной сетке
#[derive(Resource, Default)]
pub struct CursorPosition(pub Vec2);

/// Получение координат чанка по глобальной координатной системе
pub fn cursor_track(
    mut cursor_pos:     ResMut<CursorPosition>,
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
        cursor:         Res<CursorPosition>,
        mouse_input:    Res<ButtonInput<MouseButton>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        registry:       Res<Registry>,
    mut placer:         ResMut<CursorPlacer>,
    mut obj_event:      EventWriter<ObjectSpawn>,
    mut item_event:     EventWriter<ItemSpawn>,
    mut entity_event:   EventWriter<EntitySpawn>
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(match_type) = placer.placer.clone() {

            match match_type.0.as_str() {
                "item" => {
                    item_event.send(ItemSpawn(match_type.1, WorldSystem::get_currect_chunk_subtile(cursor.0.as_ivec2()), 1));
                },
                "object" => {
                    obj_event.send(ObjectSpawn(match_type.1, WorldSystem::get_currect_chunk_tile(cursor.0.as_ivec2())));
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
    }
}

fn create_text_placer(
    mut commands:   Commands,
    mut placer:     ResMut<CursorPlacer>
) {
    if placer.is_changed() {
        if let Some(entity) = placer.entity {
            commands.entity(entity).despawn_recursive();
            placer.entity = None
        }

        if !placer.placer.is_none() {
            if let Some(text) = placer.placer.clone() {
                placer.entity = Some(commands.spawn((
                    Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                format!("{}", text.1),
                                TextStyle {
                                    font_size: 8.0,
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
        cursor: Res<CursorPosition>,
        placer: ResMut<CursorPlacer>,
    mut text:   Query<&mut Transform, With<InfoTextPlace>>
) {
    if text.is_empty() {
        return;
    }

    if !placer.placer.is_none() {
        if let Ok(mut text) = text.get_single_mut() {
            text.translation = Vec3::new(cursor.0.x + 16.0, cursor.0.y + 16.0, 0.0);
        }
    }
}

#[derive(Component)]
pub struct Selectable {
    pub is_selected: bool
}

// ==============================
// Selector
// ==============================

/// Ресурс, отвечающий за хранение данных о выделении
#[derive(Resource)]
pub struct Selector {
    pub selector_entity: Option<Entity>,
    pub select_entity: Option<Entity>
}

// Компонент, отвечающийза определения самого выделения
#[derive(Component)]
pub struct Select;

/// Компонент-якорь, отвечающий за определение выделенного объекта
#[derive(Component)]
pub struct Selected;

fn select_object(
    mut commands:           Commands,
        cursor:             Res<CursorPosition>,
        keyboard_input:     Res<ButtonInput<KeyCode>>,
        mouse_buttons:      Res<ButtonInput<MouseButton>>,
    mut select:             ResMut<Selector>,
    mut chunk_res:          ResMut<Chunk>
) {
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        if mouse_buttons.just_pressed(MouseButton::Left) {
            let cursor_pos = cursor.0;
            let tiled_pos = WorldSystem::get_currect_chunk_tile(cursor_pos.as_ivec2());

            // Проверка наличие Entity на месте указанном месте
            // if let Some(selected_entity) = chunk_res.objects_ex.get(&tiled_pos) {
            //     if let Some(select_entity) = select.select_entity {
            //         if select_entity != *selected_entity {
            //             println!("1 - 1");
            //             commands.entity(*selected_entity).remove::<Selected>();
            //             select.select_entity = None;
            //             return;
            //         } else {
            //             println!("1 - 2l");
            //             commands.entity(*selected_entity).remove::<Selected>();
            //             select.select_entity = None;
            //             return;
            //         }
            //     }
            //     println!("2 - 1");
            //     select.select_entity = Some(*selected_entity);
            //     commands.entity(*selected_entity).insert(Selected);
            // } else {
            //     println!("2 - 2");
            //     if let Some(entity) = select.select_entity {
            //         commands.entity(entity).remove::<Selected>();
            //         select.select_entity = None;
            //     }
            // }

            if let Some(selected_entity) = chunk_res.objects_ex.get(&tiled_pos) {
                if select.select_entity != Some(*selected_entity) {
                    // Удаление старого выделения
                    if let Some(entity) = select.select_entity {
                        commands.entity(entity).remove::<Selected>();
                    }
                    // Новое выделение
                    select.select_entity = Some(*selected_entity);
                    commands.entity(*selected_entity).insert(Selected);
                } else {
                    // Снятие выделение у выделенной сущности
                    if let Some(entity) = select.select_entity {
                        commands.entity(entity).remove::<Selected>();
                        select.select_entity = None;
                    }
                    return;
                }
            } else {
                // Если нажатие было за пределами объектов, снимаем выделение
                if let Some(entity) = select.select_entity {
                    commands.entity(entity).remove::<Selected>();
                    select.select_entity = None;
                }
            }
        }
    }
}

fn selector_update(
    mut commands:   Commands,
        selected:   Query<(Entity, &Transform), Added<Selected>>,
        handle:     Res<TestTextureAtlas>,
    mut select:     ResMut<Selector>

) {
    if selected.is_empty() {
        return;
    }

    println!("selector");

    if select.selector_entity.is_none() {
        if let Ok((_, transform)) = selected.get_single() {
            let entity = commands.spawn((
                SpriteSheetBundle {
                    texture: handle.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: TestTextureAtlas::get_index("select", &handle),
                    },
                    transform: Transform {
                        translation: transform.translation,
                        scale: transform.scale,
                        ..default()
                    },
                    ..default()
                },
                Select,
                Name::new("Selector")
            )).id();
            println!("selector created");
            select.selector_entity = Some(entity);
        }
    }
}

fn selector_remove(
    mut commands:   Commands,
    mut removed:    RemovedComponents<Selected>,
    mut select:     ResMut<Selector>
) {
    if removed.is_empty() {
        return;
    }

    println!("selected remove");

    for entity in removed.read() {
        if let Some(entity) = select.selector_entity {
            commands.entity(entity).despawn_recursive();
            select.selector_entity = None;
            select.select_entity = None;

            println!("selected removed");
        }
    }
}

fn attach_to_select(
    mut commands:   Commands,
    mut selector:   Query<&mut Transform, (With<Select>, Without<Selected>)>,
        selected:   Query<&Transform, (With<Selected>, Without<Select>)>
) {
    if selector.is_empty() && selected.is_empty() {
        return;
    }

    if let Ok(selected) = selected.get_single() {
        if let Ok(mut selector) = selector.get_single_mut() {
            selector.translation = selected.translation;
            selector.scale = selected.scale;
        }
    }
}

// ==============================
// SelectBox
// ==============================

#[derive(Resource)]
pub struct SelectBox {
    pub is_active: bool,
    pub start_position: Vec2,
    pub current_position: Vec2
}

pub fn update_select_box(
    mut query: Query<(
        &Camera2d,
        &Camera,
        &GlobalTransform
    )>,
    mut select_box:     ResMut<SelectBox>,
        mouse_input:    Res<ButtonInput<MouseButton>>,
        windows:        Query<&Window, With<PrimaryWindow>>
) {
    let (_, camera, global_transform) = query.single_mut();

    // let window = if let RenderTarget::Window(id) = camera.target {
    //     windows.get(id).unwrap()
    // } else {
    //     windows.get_primary().unwrap()
    // };

    let window = windows.single();

    if let Some(win_pos) = window.cursor_position() {

        let window_size     = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc             = (win_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world    = global_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos       = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2       = world_pos.truncate();

        if mouse_input.just_pressed(MouseButton::Left) {
            select_box.is_active = true;
            select_box.start_position = world_pos;
            select_box.current_position = world_pos;
        }

        if mouse_input.pressed(MouseButton::Left) {
            select_box.is_active = true;
            select_box.current_position = world_pos;
        }

        if mouse_input.just_released(MouseButton::Left) {
            select_box.is_active = false;
        }
    }
}

// Test
fn delete_object(
    mut commands:       Commands,
        cursor:         Res<CursorPosition>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        handle:         Res<TestTextureAtlas>,
    mut chunk_res:      ResMut<Chunk>
) {
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        let tiled_pos = WorldSystem::get_currect_chunk_tile(cursor.0.as_ivec2());
        if let Some(entity) = chunk_res.remove_object(&tiled_pos) {
            commands.entity(entity).despawn_recursive();
        }
    }
}