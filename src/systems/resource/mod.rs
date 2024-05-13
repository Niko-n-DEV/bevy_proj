pub mod graphic;
pub mod Registry;

use std::path::Path;
use std::fs;

use bevy::prelude::*;
use extol_sprite_layer::*;

use crate::core::{
    Entity::{
        EntityType,
        HumonoidType
    },
    Settings::Settings,
    AppState,
};


#[derive(Debug, Copy, Clone, Component, PartialEq, Eq, Hash)]
pub enum SpriteLayer {
    Object,
    Item,
    Entity,
}

impl LayerIndex for SpriteLayer {
    fn as_z_coordinate(&self) -> f32 {
        use SpriteLayer::*;
        match *self {
            // Note that the z-coordinates must be at least 1 apart...
            //Background => 0.,
            Object => 1.,
            Item => 1.,
            Entity => 1.,
            //Ui => 995.
        }
    }
}

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            // Init Plugins
            .add_plugins(SpriteLayerPlugin::<SpriteLayer>::default())
            // Проверка целостности данных ядра
            // Инициализация загрузки ресурсов ядра ==============================
                // Взятие ресурсов из assets
            .add_systems(OnEnter(AppState::ResourceCheck), graphic::Graphic::load_resource_folder)
                // Определение атласов
            .insert_resource(graphic::Atlas::AtlasRes::init())
            .insert_resource(graphic::Atlas::TestTextureAtlas::default())
            .insert_resource(graphic::Atlas::DirectionAtlas::default())
            .insert_resource(SpriteLayerOptions { y_sort: true })
            // Init registry
            .insert_resource(Registry::Registry::new())
                // Проверка ресурсов на зависимости (Непонятно как оно точно работает)
            .add_systems(Update,graphic::Graphic::check_textures.run_if(in_state(AppState::ResourceCheck)))
            .add_systems(OnEnter(AppState::ResourceLoading), (
                graphic::Graphic::setup_ex,
                Self::register_types,
                Self::loading
            ))
            // - Загрузка DLC
            // Инициализация загрузки пользовательских ресурсов (Текстуры, аддоны)
            
            // Init Resource
            .add_systems(OnEnter(AppState::MainMenu), Self::setup)
        ;
    }
}

impl ResourcePlugin {
    fn setup(mut commands: Commands) {
        commands.insert_resource(Settings::load())


    }

    fn register_types(
        mut register:   ResMut<Registry::Registry>,
    ) {
        register.register_entity("human".to_string(), Registry::EntityRegistry {
            id_name: "human".to_string(),
            entity_type: EntityType::Humonoid(HumonoidType::Human),
            id_texture: 0
        });

        register.register_test("gun".to_string(), Registry::TestRegistry("gun".to_string()));       // Предмет
        register.register_test("bullet".to_string(), Registry::TestRegistry("bullet".to_string())); // Пре
        register.register_test("bullet_p".to_string(), Registry::TestRegistry("bullet_p".to_string())); // Партикл
    }

    #[allow(unused)]
    fn loading(
        mut register:   ResMut<Registry::Registry>,
    ) {
        /*
            прогон по папкам и рекурсивно внутри папок.
            поиск json файлов и их регистрация. Регистрация с последующим парсингом.
        */ 
        
        let textere_path: Vec<String> = Vec::new();

        if let Ok(entries) = fs::read_dir("Data") {
            println!("Reading Data...");

            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_dir() && path.file_name() == Some("Core".as_ref()) {
                        if let Err(err) = Self::process_loading(&path) {
                            panic!("Ошибка при обработке директории Core: {}", err)
                        }
                    }
                }
            }
        } else {
            panic!("Ошибка чтения! Проверте целостность данных!")
        }
    }

    fn process_loading(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(path)? {
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                if entry_path.is_dir() {
                    Self::process_directory(&entry_path)?;
                }
            }
        }

        Ok(())
    }

    fn process_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;

            let path = entry.path();

            if path.is_dir() {
                Self::process_directory(&path)?;
            } else {
                println!("Файл: {:?}", path);
            }
        }

        Ok(())
    }
}
