#![allow(unused)]
use bevy::{
    prelude::*, 
    window::PrimaryWindow
};
use bevy_egui::egui::epaint::text::cursor;
use bevy_rapier2d::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    resource::graphic::Atlas::TestTextureAtlas,
    AppState,
    world::World::WorldSystem,
    world::chunk::Chunk::Chunk,
    Camera::UserCamera,

    Object::EntityObject,
    items::ItemType::{
        Pickupable,
        ItemType,
        Item
    }
};


#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct User {
    pub uid: usize,
    pub user_name: String,
    pub control_entity: Option<Entity>
}

impl Default for User {
    fn default() -> Self {
        Self {
            uid: 0,
            user_name: "Niko_n".to_string(),
            control_entity: None
        }
    }
}

impl User {
    pub fn user_is_controled_entity(
        user: &User
    ) -> bool {
        if user.control_entity != None {
            true
        } else {
            false
        }
    }

    pub fn set_entity_to_control(
        user: &mut User,
        entity: Entity
    ) {
        user.control_entity = Some(entity);
    }
}

// Поместить сюда UserPlugin
pub struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app
            // Регистрация типа "User" для индексации параметров в Инспекторе
            .register_type::<User>()
            // Использование данных о позиции курсора из CursorPosition
            .init_resource::<CursorPosition>()
            .insert_resource(Selector { selector_entity: None, select_entity: None })
            // Обновление информации о позиции курсора
            .add_systems(PreUpdate, cursor_track.run_if(in_state(AppState::Game)))
            .add_systems(Update, 
                (
                    place_wall.run_if(in_state(AppState::Game)),
                    delete_wall.run_if(in_state(AppState::Game)),
                    select_object.run_if(in_state(AppState::Game)),
                    selector_update.run_if(in_state(AppState::Game)),
                    selector_remove.run_if(in_state(AppState::Game)),
                    attach_to_select.run_if(in_state(AppState::Game)),
                    spawn_bullets.run_if(in_state(AppState::Game))
                )
            )
        ;
    }
}

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

/// Компонент отвечающий за хранение информации о распалогаемом объекте на месте курсора
#[derive(Resource)]
pub struct CursorPlacer;

#[derive(Component)]
pub struct Selectable {
    pub is_selected: bool
}

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


// Test
/// Установка стены
fn place_wall(
    mut commands:       Commands,
        cursor:         Res<CursorPosition>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        _buttons:       Res<ButtonInput<MouseButton>>,
        handle:         Res<TestTextureAtlas>,
    mut chunk_res:      ResMut<Chunk>
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        let cursor_pos = cursor.0;
        let tiled_pos = WorldSystem::get_currect_chunk_tile(cursor_pos.as_ivec2());
        if !chunk_res.objects.contains_key(&tiled_pos) {
            let wall = commands
            .spawn((
                SpriteSheetBundle {
                    texture: handle.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: TestTextureAtlas::get_index("wall", &handle),
                    },
                    transform: Transform {
                        translation: Vec3::new(tiled_pos.x as f32 * 16. + 8., tiled_pos.y as f32 * 16. + 8., 0.),
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Fixed
            ))
            .insert(Collider::cuboid(8., 8.))
            .insert(Name::new("Wall"))
            .id();

            chunk_res.objects.insert(tiled_pos, wall);
        }
    }
}

fn delete_wall(
    mut commands:       Commands,
        cursor:         Res<CursorPosition>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        handle:         Res<TestTextureAtlas>,
    mut chunk_res:      ResMut<Chunk>
) {
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        let cursor_pos = cursor.0;
        let tiled_pos = WorldSystem::get_currect_chunk_tile(cursor_pos.as_ivec2());
        if let Some(entity) = chunk_res.remove_object(&tiled_pos) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_bullets(
    mut commands:       Commands,
        cursor:         Res<CursorPosition>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        handle:         Res<TestTextureAtlas>,
    mut chunk_res:      ResMut<Chunk>
) {
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        let cursor_pos = cursor.0;
        let tiled_pos = WorldSystem::get_currect_chunk_tile(cursor_pos.as_ivec2());

        let entity = commands
            .spawn((
                EntityObject {
                    ..default()
                },
                SpriteSheetBundle {
                    texture: handle.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: TestTextureAtlas::get_index("bullet", &handle)
                    },
                    transform: Transform {
                        translation: Vec3::new(tiled_pos.x as f32 * 16. + 8., tiled_pos.y as f32 * 16. + 8., 0.),
                        ..default()
                    },
                    ..default()
                }
            ))
            .insert(Pickupable {
                item: ItemType::Item(Item::Ammo),
                count: 32
            })
            .insert(Name::new("Item"))
            .id();

        chunk_res.objects_ex.insert(tiled_pos, entity);
    }
}