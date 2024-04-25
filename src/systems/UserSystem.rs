#![allow(unused)]
use bevy::{
    prelude::*, 
    render::camera::RenderTarget, 
    window::PrimaryWindow
};
use bevy_rapier2d::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

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
    Missile::*,
    Movement::DirectionState,
    world::World::WorldSystem,
    world::chunk::Chunk::Chunk,
    Camera::UserCamera
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
            // Обновление информации о позиции курсора
            .add_systems(PreUpdate, cursor_track.run_if(in_state(AppState::Game)))
            .add_systems(Update, place_wall.run_if(in_state(AppState::Game)))
        ;
    }
}

/// Позиция курсора на глобальной координатной сетке
#[derive(Resource, Default)]
pub struct CursorPosition(pub Vec2);

/// Получение координат чанка по глобальной координатной системе
pub fn cursor_track(
    mut cursor_pos: ResMut<CursorPosition>,
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
        //eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}

#[derive(Component)]
pub struct Selectable {
    pub is_selected: bool
}

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
    mut select_box: ResMut<SelectBox>,
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
    /// Установка стены
fn place_wall(
    mut commands:   Commands,
    cursor:         Res<CursorPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    _buttons:       Res<ButtonInput<MouseButton>>,
    handle:         Res<TestTextureAtlas>,
    mut chunk_res:  ResMut<Chunk>
) {
    let cursor_pos = cursor.0;
    if keyboard_input.just_pressed(KeyCode::KeyE) {
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