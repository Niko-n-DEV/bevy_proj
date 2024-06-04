use bevy::prelude::*;

use crate::core::{
    Entity::EntityBase,
    resource::graphic::Atlas::{
        AtlasRes, 
        AtlasType
    }, 
    world::{
        chunk::Chunk::Chunk, 
        World::WorldSystem
    }, 
    UserSystem::CursorPosition,
    AppState
};

pub fn select_plugin(app: &mut App) {
    // Init resource
    app.init_resource::<SelectorRes>();
    // Init systems
    app.add_systems(Update, 
        (
            select_object,
            selector_add,
            selector_remove,
            attach_to_select
        ).run_if(in_state(AppState::Game))
    );
}

// ==============================
// Selector
// ==============================

/// Ресурс, отвечающий за хранение данных о выделении
#[derive(Resource, Default)]
pub struct SelectorRes {
    pub selector_entity: Option<Entity>,
    pub select_entity: Option<Entity>
}

/// Компонент, отвечающийза определения самого выделения
#[derive(Component)]
pub struct Select;

/// Компонент-якорь, отвечающий за определение выделенного объекта
#[derive(Component)]
pub struct Selected;

// Определения выделения
// Установка выделения на определённом объекте
// Переустановка выделения на объект
// Удаление выделения при тыке на место без объектов
fn select_object(
    mut commands:           Commands,
        entities:           Query<(Entity ,&Transform), With<EntityBase>>,
        cursor:             Res<CursorPosition>,
        keyboard_input:     Res<ButtonInput<KeyCode>>,
        mouse_buttons:      Res<ButtonInput<MouseButton>>,
    mut select:             ResMut<SelectorRes>,
        chunk_res:          Res<Chunk>
) {
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        if mouse_buttons.just_pressed(MouseButton::Left) {

            if let Some(selected_entity) = chunk_res.objects_ex.get(&&WorldSystem::get_currect_chunk_subtile(cursor.0.as_ivec2())) {
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
                    return;
                }
            }

            for entity in &entities {
                if 8.0 > Vec3::distance(entity.1.translation, cursor.0.extend(0.5)) {
                    if select.select_entity != Some(entity.0) {
                        // Удаление старого выделения
                        if let Some(selected_entity) = select.select_entity {
                            commands.entity(selected_entity).remove::<Selected>();
                        }
                        // Новое выделение
                        select.select_entity = Some(entity.0);
                        commands.entity(entity.0).insert(Selected);
                    } else {
                        // Снятие выделения у выделенной сущности
                        if let Some(selected_entity) = select.select_entity {
                            commands.entity(selected_entity).remove::<Selected>();
                            select.select_entity = None;
                        }
                    }
                    return;
                }
            }
        }
    }
}

// Создание выделения при установке якоря
fn selector_add(
    mut commands:   Commands,
        selected:   Query<(Entity, &Transform), Added<Selected>>,
        atlas:      Res<AtlasRes>,
    mut select:     ResMut<SelectorRes>

) {
    if selected.is_empty() {
        return;
    }

    if select.selector_entity.is_none() {
        if let Ok((_, transform)) = selected.get_single() {
            if let Some(img) = atlas.get_texture(AtlasType::Ui, "select") {
                let entity = commands.spawn((
                    SpriteBundle {
                        texture: img.1.clone(),
                        transform: Transform {
                            translation: Vec3::new(transform.translation.x, transform.translation.y, 100.),
                            scale: transform.scale,
                            ..default()
                        },
                        ..default()
                    },
                    img.0,
                    Select,
                    Name::new("Selector")
                )).id();

                select.selector_entity = Some(entity);
            } else {
                warn!("The selection could not be set!")
            }
        }
    }
}

// Удаление выделения при удалении якоря
fn selector_remove(
    mut commands:   Commands,
    mut removed:    RemovedComponents<Selected>,
    mut select:     ResMut<SelectorRes>
) {
    if removed.is_empty() {
        return;
    }

    for entity in removed.read() {
        if let Some(selected_entity) = select.selector_entity {
            if entity == selected_entity {  // Возможная проблема мульти компонента
                commands.entity(selected_entity).despawn_recursive();
                select.selector_entity = None;
                select.select_entity = None;

            } else if select.select_entity.is_none() {
                commands.entity(selected_entity).despawn_recursive();
                select.selector_entity = None;
            }
        }
    }
}

fn attach_to_select(
    mut selector:   Query<&mut Transform, (With<Select>, Without<Selected>)>,
        selected:   Query<&Transform, (With<Selected>, Without<Select>)>
) {
    if selector.is_empty() && selected.is_empty() {
        return;
    }

    if let Ok(selected) = selected.get_single() {
        if let Ok(mut selector) = selector.get_single_mut() {
            selector.translation = Vec3::new(selected.translation.x, selected.translation.y, 100.);
            selector.scale = selected.scale;
        }
    }
}
