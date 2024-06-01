pub mod graphic;
pub mod Registry;

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::fs;

use bevy::{
    asset::LoadedFolder, 
    prelude::*
};

use extol_sprite_layer::*;

use crate::core::{
    // EntityType::{
    //     EntityType,
    //     HumonoidType
    // },
    Settings::Settings,
    AppState,
};

#[allow(unused)]
#[derive(Debug, Copy, Clone, Component, PartialEq, Eq, Hash)]
pub enum SpriteLayer {
    Object,
    Item,
    Entity,
    EntityPart
}

impl LayerIndex for SpriteLayer {
    fn as_z_coordinate(&self) -> f32 {
        use SpriteLayer::*;
        match *self {
            // Note that the z-coordinates must be at least 1 apart...
            //Background => 0.,
            Object => 1.1,
            Item => 1.,
            Entity => 1.,
            EntityPart => 1.0,
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
            .add_systems(OnEnter(AppState::ResourceCheck), Self::start_loading)
            // Определение атласов
            .insert_resource(graphic::Atlas::TestTextureAtlas::default())
            .insert_resource(graphic::Atlas::DirectionAtlas::default())
            // Init registry
            .insert_resource(Registry::Registry::new())
            // Проверка ресурсов на зависимости (Непонятно как оно точно работает)
            .add_systems(Update,graphic::check_textures.run_if(in_state(AppState::ResourceCheck)))
            .add_systems(OnEnter(AppState::ResourceLoading), (
                Self::loading_module,
                graphic::setup_ex,
                Self::register_types,
            ).chain())
            // - Загрузка DLC
            // Инициализация загрузки пользовательских ресурсов (Текстуры, аддоны)
            
            // Init Resource
            .add_systems(OnEnter(AppState::MainMenu), Self::load_settings)
        ;
    }
}


// /// Ресурс хранящий в себе загружаемую папку ресурсов
// #[derive(Resource, Default)]
// pub struct ResourceFolder(Handle<LoadedFolder>, Handle<LoadedFolder>);

#[derive(Resource, Default)]
pub struct ResourceModule(Handle<LoadedFolder>);

#[derive(Resource, Default)]
pub struct LoadingBuffer {
    source_id:                  String,
    textures_path_buf:          Vec<String>,
    reg_item_tex_path:          Vec<String>,
    reg_entity_tex_path:        Vec<String>,
    reg_object_tex_path:        Vec<String>,
    reg_object_ct_tex_path:     Vec<String>,
    reg_ui_tex_path:            Vec<String>,
    verified_item_texture:      Vec<String>,
    verified_entity_texture:    Vec<String>,
    verified_object_texture:    Vec<String>,
    verified_object_ct_texture: Vec<String>,
    verified_ui_texture:        Vec<String>,
}

impl ResourcePlugin {

    /// функция для загрузки ресурсов из определённой папки, по умолчанию эта папка - assets, и всё его содержимое
    pub fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
        info!("Insert resources");
        // commands.insert_resource(ResourceFolder(
        //     asset_server.load_folder(""),
        //     asset_server.load_folder("core/textures/entity/player"),
        // ));
        
        commands.insert_resource(ResourceModule(asset_server.load_folder("Data://Core/Textures")));

        commands.insert_resource(LoadingBuffer::default());

        commands.insert_resource(graphic::Atlas::AtlasRes::init());

        info!("State: ResourceCheck");
    }

    fn load_settings(mut commands: Commands) {
        commands.insert_resource(Settings::load())
    }

    #[allow(unused)]
    fn register_types(
        mut register:   ResMut<Registry::Registry>,
    ) {
        // register.register_entity(Registry::EntityRegistry {
        //     id_name:        "human".to_string(),
        //     id_source:      Some("core".to_string()),
        //     id_texture_b:   "human".to_string(),
        //     id_texture_h:   None,
        //     entity_type:    EntityType::Humonoid(HumonoidType::Human),
        //     health:         100.0
        // });

        // register.register_test("gun".to_string(), Registry::TestRegistry("gun".to_string()));       // Предмет
        // register.register_test("bullet".to_string(), Registry::TestRegistry("bullet".to_string())); // Пре
        // register.register_test("bullet_p".to_string(), Registry::TestRegistry("bullet_p".to_string())); // Партикл
    }

    fn loading_module(
        // mut commands:       Commands,
        mut register:       ResMut<Registry::Registry>,
        mut load_buff:      ResMut<LoadingBuffer>
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

        // commands.remove_resource::<LoadingBuffer>();
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
                        "Textures" => {
                            let res_path = path.join("entities");
                            if res_path.exists() {
                                Self::process_directory_assets(&mut register, &mut load_buff.textures_path_buf, &res_path)?;
                            }

                            let assets_path = path.join("items");
                            if assets_path.exists() {
                                Self::process_directory_assets(&mut register, &mut load_buff.textures_path_buf, &assets_path)?;
                            }

                            let assets_path = path.join("objects");
                            if assets_path.exists() {
                                Self::process_directory_assets(&mut register, &mut load_buff.textures_path_buf, &assets_path)?;
                            }

                            let assets_path = path.join("objects_ct");
                            if assets_path.exists() {
                                Self::process_directory_assets(&mut register, &mut load_buff.textures_path_buf, &assets_path)?;
                            }

                            let assets_path = path.join("gui");
                            if assets_path.exists() {
                                Self::process_directory_assets(&mut register, &mut load_buff.reg_ui_tex_path, &assets_path)?;
                            }
                        },
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

                            let res_path = path.join("objects_ct");
                            if res_path.exists() {
                                Self::process_directory_res(&mut register, &mut load_buff, &res_path)?;
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }

        load_buff.verified_item_texture         = Self::process_assets(&load_buff.reg_item_tex_path, &load_buff.textures_path_buf);
        load_buff.verified_entity_texture       = Self::process_assets(&load_buff.reg_entity_tex_path, &load_buff.textures_path_buf);
        load_buff.verified_object_texture       = Self::process_assets(&load_buff.reg_object_tex_path, &load_buff.textures_path_buf);
        load_buff.verified_object_ct_texture    = Self::process_assets(&load_buff.reg_object_ct_tex_path, &load_buff.textures_path_buf);

        load_buff.verified_ui_texture           = Self::process_ui_assets(&load_buff.reg_ui_tex_path);
        
        Ok(())
    }

    /// Проверка соответствия текстур с запрашиваемыми текстурами объектами регистра
    fn process_assets(
        find:  &Vec<String>,
        check: &Vec<String>
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for check_string in find {
            let file_find = PathBuf::from(check_string);
            for tex_path in check {
                let path_buf = PathBuf::from(tex_path);
                if let Some(file_stem) = path_buf.file_stem() {
                    if file_stem == file_find.file_stem().unwrap() {
                        let to_load = path_buf.to_string_lossy().into_owned();
                        if !result.contains(&to_load) {
                            result.push(path_buf.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
        result
    }
    
    /// Проверка соответствия текстур с заявленными текстурами интерфейса проекта
    fn process_ui_assets(check: &Vec<String>) -> Vec<String> {
        // Фиксированный список необходимых текстур
        let required_textures = vec![
            "about_avatar_ui_btn".to_string(),
            "inv_ui_btn".to_string(),
            "crafting_ui_btn".to_string(),
            "select".to_string(),
        ];
    
        // Преобразуем фиксированный список в HashSet для эффективного поиска
        let required_set: HashSet<_> = required_textures.iter().collect();
    
        // Результирующий вектор для валидных текстур
        let mut valid_textures = Vec::new();
    
        // Преобразуем список на проверку и проверяем наличие в фиксированном списке
        for tex_path in check {
            if let Some(file_stem) = PathBuf::from(tex_path).file_stem().and_then(|stem| stem.to_str()) {
                if required_set.contains(&file_stem.to_string()) {
                    valid_textures.push(tex_path.clone());
                }
            }
        }
    
        valid_textures
    }

    fn process_directory_assets(
        mut register:   &mut Registry::Registry,
        mut load_buff:  &mut Vec<String>,
            dir:        &Path
    ) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
    
            let path = entry.path();
    
            if path.is_file() && path.extension().map_or(false, |ext| ext == "png") {
                let path_str = path.to_string_lossy().into_owned();
                
                if !load_buff.contains(&path_str) {
                    load_buff.push(path_str);
                }
            } else if path.is_dir() {
                Self::process_directory_assets(&mut register, &mut load_buff, &path)?;
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

                            load_buff.reg_entity_tex_path.push(module.id_texture_b.clone());
                            if !module.id_texture_h.is_none() {
                                if let Some(texture_h) = module.id_texture_h.clone() {
                                    load_buff.reg_entity_tex_path.push(texture_h);
                                }
                            }

                            register.register_entity(Registry::EntityRegistry {
                                id_name:        module.id_name,
                                id_source:      Some(load_buff.source_id.clone()),
                                id_texture_b:   module.id_texture_b,
                                id_texture_h:   module.id_texture_h,
                                entity_type:    module.entity_type,
                                health:         module.health
                            });
                        }
                    }
                }
                // Обработка json файлов определяющие предметы
                if dir.file_name().map_or(false, |name| name == "items") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        if let Ok(module) = serde_json::from_str::<Registry::ItemRegistry>(&contents) {

                            load_buff.reg_item_tex_path.push(module.id_texture.clone());

                            register.register_item(Registry::ItemRegistry {
                                id_name:    module.id_name,
                                id_source:  Some(load_buff.source_id.clone()),
                                id_texture: module.id_texture,
                                item_type:  module.item_type,
                                item_size:  module.item_size,
                                stackable:  module.stackable,
                                stack_size: module.stack_size,
                                durability: module.durability
                            });
                        }
                    }
                    
                }

                // Обработка json файлов определяющие объекты
                if dir.file_name().map_or(false, |name| name == "objects") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        if let Ok(module) = serde_json::from_str::<Registry::ObjectRegistry>(&contents) {

                            load_buff.reg_object_tex_path.push(module.id_texture.clone());

                            register.register_object(Registry::ObjectRegistry {
                                id_name:        module.id_name,
                                id_source:      Some(load_buff.source_id.clone()),
                                id_texture:     module.id_texture,
                                size:           module.size,
                                collision:      module.collision,
                                durability:     module.durability
                            });
                        }
                    }
                    
                }

                // Обработка json файлов определяющие объекты с соединяющимися текстурами
                if dir.file_name().map_or(false, |name| name == "objects_ct") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        if let Ok(module) = serde_json::from_str::<Registry::PersistentObjectRegistry>(&contents) {

                            load_buff.reg_object_ct_tex_path.push(module.id_texture.clone());

                            register.register_object_ct(Registry::PersistentObjectRegistry {
                                id_name:        module.id_name,
                                id_source:      Some(load_buff.source_id.clone()),
                                id_texture:     module.id_texture,
                                size:           module.size,
                                collision:      module.collision,
                                durability:     module.durability
                            });
                        }
                    }
                    
                }
            } else if path.is_dir() {
                Self::process_directory_res(&mut register, &mut load_buff, &path)?;
            }
        }
    
        Ok(())
    }
}
