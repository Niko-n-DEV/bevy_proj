pub mod graphic;
pub mod Registry;

use std::path::Path;
use std::fs;

use bevy::prelude::*;
use extol_sprite_layer::*;

use crate::core::{
    EntityType::{
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
            // Insert Resources
            .insert_resource(LoadingBuffer::default())
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

#[derive(Resource, Default)]
struct LoadingBuffer {
    source_id: String,
    textures_path_buffet: Vec<String>
}

impl ResourcePlugin {
    fn setup(mut commands: Commands) {
        commands.insert_resource(Settings::load())
        

    }

    fn register_types(
        mut register:   ResMut<Registry::Registry>,
    ) {
        register.register_entity(Registry::EntityRegistry {
            id_name: "human".to_string(),
            id_source: Some("core".to_string()),
            entity_type: EntityType::Humonoid(HumonoidType::Human),
            id_texture: 0
        });

        register.register_test("gun".to_string(), Registry::TestRegistry("gun".to_string()));       // Предмет
        register.register_test("bullet".to_string(), Registry::TestRegistry("bullet".to_string())); // Пре
        register.register_test("bullet_p".to_string(), Registry::TestRegistry("bullet_p".to_string())); // Партикл
    }

    fn loading(
        mut commands:  Commands,
        mut register:  ResMut<Registry::Registry>,
        mut load_buff: ResMut<LoadingBuffer>
    ) {
        /*
            прогон по папкам и рекурсивно внутри папок.
            поиск json файлов и их регистрация. Регистрация с последующим парсингом.
        */ 
        
        // let textere_path: Vec<String> = Vec::new();

        if let Ok(entries) = fs::read_dir("Data") {
            println!("Reading Data...");

            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_dir() && path.file_name() == Some("Core".as_ref()) {
                        println!("Reading Core...");
                        if let Err(err) = Self::process_loading(&mut register, &mut load_buff, &path) {
                            panic!("Ошибка при обработке директории Core: {}", err)
                        }
                    }
                }
            }
        }

        commands.remove_resource::<LoadingBuffer>();
    }

    fn process_loading(
        mut register:   &mut Registry::Registry,
        mut load_buff:  &mut LoadingBuffer,
            path:       &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Регистрация модуля
        let modul_path = path.join("mod.json");
        if modul_path.exists() {
            if let Ok(contents) = fs::read_to_string(&modul_path) {
                if let Ok(module) = serde_json::from_str::<Registry::ModuleRegistry>(&contents) {
                    load_buff.source_id = module.id.clone();
                    register.register_module(module);
                    println!("Модуль зарегистрирован!");
                }
            }
        } else {
            return Err("Нет файла регистрации модуля!".into());
        }

        for entry in fs::read_dir(path)? {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Проверка директорий
                if path.is_dir() {
                    let dir_name = path.file_name().ok_or("Невалидное имя директории")?;
                    let dir_name = dir_name.to_string_lossy();

                    match dir_name.as_ref() {
                        "Defs" => {
                            let res_path = path.join("entities");
                            if res_path.exists() {
                                Self::process_directory_res(&mut register, &mut load_buff, &res_path)?;
                            }

                            let res_path = path.join("items");
                            if res_path.exists() {
                                Self::process_directory_res(&mut register, &mut load_buff, &res_path)?;
                            }

                            let res_path = path.join("objects");
                            if res_path.exists() {
                                Self::process_directory_res(&mut register, &mut load_buff, &res_path)?;
                            }
                        }
                        "Textures" => {
                            let assets_path = path.join("items");
                            if assets_path.exists() {
                                Self::process_directory_assets(&mut register, &mut load_buff, &assets_path)?;
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }

        Ok(())
    }

    fn process_directory_assets(
        mut register:   &mut Registry::Registry,
        mut load_buff:  &mut LoadingBuffer,
            dir:        &Path
    ) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
    
            let path = entry.path();
    
            if path.is_file() {
                let path_str = path.to_string_lossy().into_owned();
                load_buff.textures_path_buffet.push(path_str)
            } else if path.is_dir() {
                //let dir_name = path.file_name().ok_or("Невалидное имя директории")?;
                //let dir_name = dir_name.to_string_lossy();
                
                Self::process_directory_assets(&mut register, &mut load_buff, &path)?;

                // match dir_name.as_ref() {
                //     "textures" => {
                //         let textures_items_path = path.join("items");
                //         if textures_items_path.exists() {
                //             Self::process_directory_assets(&mut register, &mut load_buff, &textures_items_path)?;
                //         }

                //         let textures_entities_path = path.join("entities");
                //         if textures_entities_path.exists() {
                //             Self::process_directory_assets(&mut register, &mut load_buff, &textures_entities_path)?;
                //         }
                //     },
                //     _ => {
                //         Self::process_directory_assets(&mut register, &mut load_buff, &path)?;
                //     }
                // }
            }
        }
    
        Ok(())
    }

    fn process_directory_res(
        mut register:   &mut Registry::Registry,
        mut load_buff:  &mut LoadingBuffer,
            dir:        &Path
    ) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
    
            let path = entry.path();
    
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                // Обработка json файлов определяющих сущности
                if dir.file_name().map_or(false, |name| name == "entities") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        if let Ok(module) = serde_json::from_str::<Registry::EntityRegistry>(&contents) {
                            register.register_entity(Registry::EntityRegistry {
                                id_name: module.id_name,
                                id_source: Some(load_buff.source_id.clone()),
                                entity_type: module.entity_type,
                                id_texture: module.id_texture
                            });
                        }
                    }
                }
                // Обработка json файлов определяющие предметы
                if dir.file_name().map_or(false, |name| name == "items") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        if let Ok(module) = serde_json::from_str::<Registry::ItemRegistry>(&contents) {
                            register.register_item(Registry::ItemRegistry {
                                id_name: module.id_name,
                                id_source: Some(load_buff.source_id.clone()),
                                item_type: module.item_type
                            });
                        }
                    }
                    
                }

                // Обработка json файлов определяющие объекты
                if dir.file_name().map_or(false, |name| name == "objects") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        if let Ok(module) = serde_json::from_str::<Registry::ObjectRegistry>(&contents) {
                            register.register_object(Registry::ObjectRegistry {
                                id_name: module.id_name,
                                id_source: Some(load_buff.source_id.clone()),
                                //size_type: module.size_type,
                                size: module.size,
                                collision: module.collision
                            });
                        }
                    }
                    
                }
            } else if path.is_dir() {
                let dir_name = path.file_name().ok_or("Невалидное имя директории")?;
                let dir_name = dir_name.to_string_lossy();
    
                match dir_name.as_ref() {
                    "entities" | "items" | "objects" => {
                        Self::process_directory_res(&mut register, &mut load_buff, &path)?;
                    },
                    _ => continue,
                }
            }
        }
    
        Ok(())
    }
}
