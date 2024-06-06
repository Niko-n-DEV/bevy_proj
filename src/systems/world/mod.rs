#![allow(non_snake_case)]
pub mod chunk;
pub mod Grid;
pub mod TileMap;
pub mod World;

//pub mod chunk;

use bevy::prelude::*;

use crate::core::{
    Entity::EntityBase,
    Object::EntityObject,
    Item::EntityItem,
    world::TileMap::TileM,
    AppState
};

pub struct WorldTaskManager;

// impl Plugin for WorldTaskManager {
//     fn build(&self, app: &mut App) {
//         app
//             .add_systems(OnEnter(AppState::LoadingInGame), Self::load_data)
//         ;
//     }
// }

/*
    1. Происходи инициализация загрузки мира
    1.1 Поиск файлов мира и их загрузка
    1.2 Прогрузка начальной области (территории, объектов и сущностей)
    1.3 Прочая прогрузка (как игрок и т.д.)
    2. Выгрузка мира
    2.1 Сохранение мира в файл путём парсинга (сериализации данных в тот или иной вид)
*/

#[allow(unused)]
impl WorldTaskManager {
    /// Функция для загрузки данных мира
    pub fn load_data(mut next_state: ResMut<NextState<AppState>>) {
        next_state.set(AppState::Game);
        info!("State: Game")
    }

    /// Функция для загрузки и расположения объектов
    pub fn load_object() {}

    pub fn place_object(
        position: IVec2,
        object: Entity
    ) {

    }

    /// Функция для загрузки и расположения ентити
    pub fn load_entity() {}

    /// функция выгрузки объектов
    fn despawn_object(
        mut commands: Commands,
        mut objects: Query<Entity, With<EntityObject>>,
    ) {
        if objects.is_empty()  {
            return;
        }
    
        for obj in objects.iter_mut() {
            commands.entity(obj).despawn_recursive();
        }
    }

    /// функция выгрузки предметов
    fn despawn_items(
        mut commands: Commands,
        mut items: Query<Entity, With<EntityItem>>,
    ) {
        if items.is_empty()  {
            return;
        }
    
        for item in items.iter_mut() {
            commands.entity(item).despawn_recursive();
        }
    }

    /// функция выгрузки ентити
    fn despawn_entities(
        mut commands: Commands,
        mut entities: Query<Entity, With<EntityBase>>,
    ) {
        if entities.is_empty()  {
            return;
        }
    
        for entity in entities.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn despawn_terrain(
        mut commands: Commands,
        mut entities: Query<Entity, With<TileM>>
    ) {
        if entities.is_empty()  {
            return;
        }

        for entities in entities.iter_mut() {
            commands.entity(entities).despawn_recursive();
        }
    }

    /// Универсальная функция сохранения
    pub fn discharge_and_save() {}
}
