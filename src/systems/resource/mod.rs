pub mod graphic;

use bevy::prelude::*;

use crate::core::{
    resource::graphic::{
        Atlas::{DirectionAtlas, TestTextureAtlas},
        Graphic::*,
    },
    AppState,
};

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            // Проверка целостности данных ядра
            // Инициализация загрузки ресурсов ядра ==============================
                // Взятие ресурсов из assets
            .add_systems(OnEnter(AppState::ResourceCheck), load_resource_folder)
                // Определение атласов
            .insert_resource(TestTextureAtlas::default())
            .insert_resource(DirectionAtlas::default())
                // Проверка ресурсов на зависимости (Непонятно как оно точно работает)
            .add_systems(Update,check_textures.run_if(in_state(AppState::ResourceCheck)))
            .add_systems(OnEnter(AppState::ResourceLoading), setup_ex)
            // - Загрузка DLC
            // Инициализация загрузки пользовательских ресурсов (Текстуры, аддоны)
        ;
    }
}
