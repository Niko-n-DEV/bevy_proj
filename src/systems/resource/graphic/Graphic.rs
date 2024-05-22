//#![allow(unused)]
use std::{collections::HashMap, path::Path};

use bevy::{
    asset::LoadedFolder, 
    prelude::*, 
    render::texture::ImageSampler
};

use crate::core::{
    resource::{
        graphic::Atlas::{
            AtlasRes,
            DirectionAtlas, 
            TestTextureAtlas
        },
        //Registry::Registry,
        LoadingBuffer,
        ResourceFolder,
        ResourceModule
    },
    AppState,
};

/// Проверка чего-то с каждым обновлением
pub fn check_textures(
    mut next_state:         ResMut<NextState<AppState>>,
        resource_folder:    Res<ResourceFolder>,
    mut events:             EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&resource_folder.0) {
            next_state.set(AppState::ResourceLoading);
        }
    }
}

pub fn setup_ex(
    mut commands:           Commands,
        resource_handle:    Res<ResourceFolder>,
        resource_module:    Res<ResourceModule>,
    mut handle_cust_atlas:  ResMut<TestTextureAtlas>,
    mut handle_dir_atlas:   ResMut<DirectionAtlas>,
        loaded_folders:     Res<Assets<LoadedFolder>>,
    // mut register:           ResMut<Registry>,
    mut atlas:              ResMut<AtlasRes>,
    //    asset_server:       Res<AssetServer>,
    mut texture_atlases:    ResMut<Assets<TextureAtlasLayout>>,
    mut textures:           ResMut<Assets<Image>>,
    mut next_state:         ResMut<NextState<AppState>>,
    load_buff:              ResMut<LoadingBuffer>
) {
    // ==============================
    // Сборка текстур в единый атлас
    // ==============================
    let loaded_folder = loaded_folders.get(&resource_handle.0).unwrap();

    let (texture_atlas_nearest, nearest_texture, _hash_t) = create_texture_atlas(
        &loaded_folder,
        Some(UVec2::splat(1)),
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let atlas_nearest_handle = texture_atlases.add(texture_atlas_nearest);

    // Спавн спрайта атласа ждя debug
    commands.spawn(SpriteBundle {
        texture: nearest_texture.clone(),
        transform: Transform {
            translation: Vec3 { x: -128., y: -128., z: 1. },
            ..default()
        },
        ..default()
    });

    handle_cust_atlas.layout =  Some(atlas_nearest_handle.clone());
    handle_cust_atlas.image =   Some(nearest_texture.clone());
    handle_cust_atlas.ids =     Some(_hash_t.clone());

    // ==============================
    // Сборка атласов в единый атлас
    // ==============================
    let loaded_folder = loaded_folders.get(&resource_handle.1).unwrap();
    let (texture_atlas_dir_atlases, nearest_texture_atlases, _hash) = load_and_index_atlas(
        &loaded_folder,
        Some(UVec2::splat(1)),
        Some(ImageSampler::nearest()),
        &mut textures,
    );

    let atlas_dir_nearest_handle = texture_atlases.add(texture_atlas_dir_atlases);

    handle_dir_atlas.layout =   Some(atlas_dir_nearest_handle.clone());
    handle_dir_atlas.image =    Some(nearest_texture_atlases.clone());
    handle_dir_atlas.ids =      Some(_hash.clone());

    // ==============================
    // Entity
    // ==============================
    let loaded_folder = loaded_folders.get(&resource_module.0).unwrap();

    let (texture_atlas_nearest, entity_texture, entity_hash) = load_and_index_atlas_ex(
        &load_buff.verified_entity_texture,
        &loaded_folder,
        None, //Some(UVec2::splat(1)),
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let entity_layout = texture_atlases.add(texture_atlas_nearest);

    atlas.entity.layout =    Some(entity_layout);
    atlas.entity.image =     Some(entity_texture);
    atlas.entity.ids =       Some(entity_hash);

    // ==============================
    // Items
    // ==============================
    let loaded_folder = loaded_folders.get(&resource_module.0).unwrap();

    let (texture_atlas_nearest, items_texture, items_hash) = create_texture_atlas_ex(
        &load_buff.verified_item_texture,
        &loaded_folder,
        Some(UVec2::splat(1)),
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let items_layout = texture_atlases.add(texture_atlas_nearest);

    atlas.items.layout =    Some(items_layout);
    atlas.items.image =     Some(items_texture);
    atlas.items.ids =       Some(items_hash);

    // ==============================
    // Objects
    // ==============================
    let loaded_folder = loaded_folders.get(&resource_module.0).unwrap();

    let (texture_atlas_nearest, objects_texture, objects_hash) = create_texture_atlas_ex(
        &load_buff.verified_object_texture,
        &loaded_folder,
        Some(UVec2::splat(1)),
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let object_layout = texture_atlases.add(texture_atlas_nearest);

    atlas.objects.layout =  Some(object_layout);
    atlas.objects.image =   Some(objects_texture);
    atlas.objects.ids =     Some(objects_hash);

    // ==============================
    // Test
    // ==============================
    atlas.test.layout =     Some(atlas_nearest_handle);
    atlas.test.image =      Some(nearest_texture);
    atlas.test.ids =        Some(_hash_t);
    // ==============================
    // 
    // ==============================

    next_state.set(AppState::MainMenu);
    info!("State: MainMenu")
}

// ==============================
// 
// ==============================
fn create_texture_atlas_ex(
    load_buff:  &Vec<String>,
    folder:     &LoadedFolder,
    padding:    Option<UVec2>,
    sampling:   Option<ImageSampler>,
    textures:   &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>, HashMap<String, usize>) {
    let mut textures_ids: HashMap<String, usize> = HashMap::new();
    // Скорее всего создаётся полотно, в которое будет помещаться текстуры
    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().padding(padding.unwrap_or_default());

    let mut num: usize = 0;

    // Прогон по имеющимся текстурам в loadedfolder
    for handle in folder.handles.iter() {
        if let Some(path) = handle.path() {
            if let Some(file_name) = path.to_string().as_str().split('/').last() {
                if let Some(first) = file_name.rsplit(|c| c == '\\' || c == '/').next() {
                    // println!("{first}");
                    for sec in load_buff.clone() {
                        if let Some(second) = sec.rsplit(|c| c == '\\' || c == '/').next() {
                            // println!("{second}");
                            if first == second {
                                // Получение id у прогоняемой текстуры
                                let id = handle.id().typed_unchecked::<Image>();
                                // Проверка, преобразоваемый файл ли в текстуру
                                let Some(texture) = textures.get(id) else {
                                    warn!(
                                        "{:?} did not resolve to an `Image` asset.",
                                        handle.path().unwrap()
                                    );

                                    continue;
                                };

                                // Получение имени загружаемого файла, чтобы использовать это как ключь-имя в hash-таблице
                                if let Some(path) = handle.path() {
                                    if let Some(file_name) = path.to_string().as_str().split('/').last() {
                                        let file_fmt = Path::new(file_name).file_stem().unwrap().to_string_lossy();

                                        println!("Loaded module resource | {}", file_fmt);
                                        textures_ids.insert(file_fmt.to_string(), num);
                                    } else {
                                        warn!("[Error] - An error occurred while reading the file name!")
                                    }
                                } else {
                                    warn!("[Error] - An error occurred while reading the file path!")
                                }

                                num += 1;
                                // Добавление на полотно добавляемую текстуру
                                texture_atlas_builder.add_texture(Some(id), texture);
                            }
                        }
                    }
                }
            }
        }
    }

    // Финальная сборка полотна в layout и цельную текстуру
    let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
    // Добавление текстуры в handle
    let texture = textures.add(texture);

    // Обновление настройки выборки в атласе текстур
    let image = textures.get_mut(&texture).unwrap();
    // Применение обработки к текстурам
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture, textures_ids)
}

fn load_and_index_atlas_ex(
    load_buff:  &Vec<String>,
    folder:     &LoadedFolder,
    padding:    Option<UVec2>,
    sampling:   Option<ImageSampler>,
    textures:   &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>, HashMap<String, usize>) {
    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().padding(padding.unwrap_or_default());

    let mut textures_ids: HashMap<String, usize> = HashMap::new();
    let mut num: usize = 0;

    // Прогон по имеющимся текстурам в loadedfolder
    for handle in folder.handles.iter() {
        if let Some(path) = handle.path() {
            if let Some(file_name) = path.to_string().as_str().split('/').last() {
                if let Some(first) = file_name.rsplit(|c| c == '\\' || c == '/').next() {
                    // println!("{first}");
                    for sec in load_buff.clone() {
                        if let Some(second) = sec.rsplit(|c| c == '\\' || c == '/').next() {
                            // println!("{second}");
                            if first == second {
                                // Получение id у прогоняемой текстуры
                                let id = handle.id().typed_unchecked::<Image>();
                                // Проверка, преобразоваемый файл ли в текстуру
                                let Some(texture) = textures.get(id) else {
                                    warn!(
                                        "{:?} did not resolve to an `Image` asset.",
                                        handle.path().unwrap()
                                    );

                                    continue;
                                };

                                // Получение имени загружаемого файла, чтобы использовать это как ключь-имя в hash-таблице
                                if let Some(path) = handle.path() {
                                    if let Some(file_name) = path.to_string().as_str().split('/').last() {
                                        let file_fmt = Path::new(file_name).file_stem().unwrap().to_string_lossy();

                                        println!("Loaded module resource | {}", file_fmt);
                                        textures_ids.insert(file_fmt.to_string(), num);
                                    } else {
                                        warn!("[Error] - An error occurred while reading the file name!")
                                    }
                                } else {
                                    warn!("[Error] - An error occurred while reading the file path!")
                                }

                                num += 1;
                                // Добавление на полотно добавляемую текстуру
                                texture_atlas_builder.add_texture(Some(id), texture);
                            }
                        }
                    }
                }
            }
        }
    }

    let (_, texture) = texture_atlas_builder.finish().unwrap();
    let texture = textures.add(texture);

    // Обновление настройки выборки в атласе текстур
    let image = textures.get_mut(&texture).unwrap();
        image.sampler = sampling.unwrap_or_default();

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(16. as f32, 16. as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );

    (layout, texture, textures_ids)
}
// ==============================
// 
// ==============================

/// Создание атласа текстур с заданными настройками заполнения и выборки из отдельных спрайтов в данной папке
fn create_texture_atlas(
    folder:     &LoadedFolder,
    padding:    Option<UVec2>,
    sampling:   Option<ImageSampler>,
    textures:   &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>, HashMap<String, usize>) {
    let mut textures_ids: HashMap<String, usize> = HashMap::new();
    // Скорее всего создаётся полотно, в которое будет помещаться текстуры
    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().padding(padding.unwrap_or_default());

    let mut num: usize = 0;

    // Прогон по имеющимся текстурам в loadedfolder
    for handle in folder.handles.iter() {
        // Получение id у прогоняемой текстуры
        let id = handle.id().typed_unchecked::<Image>();
        // Проверка, преобразоваемый файл ли в текстуру
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );

            continue;
        };

        // Получение имени загружаемого файла, чтобы использовать это как ключь-имя в hash-таблице
        if let Some(path) = handle.path() {
            if let Some(file_name) = path.to_string().as_str().split('/').last() {
                // println!("{file_name}");
                let file_fmt = Path::new(file_name).file_stem().unwrap().to_string_lossy();

                println!("Loaded resource | {}", file_fmt);
                textures_ids.insert(file_fmt.to_string(), num);
            } else {
                warn!("[Error] - An error occurred while reading the file name!")
            }
        } else {
            warn!("[Error] - An error occurred while reading the file path!")
        }

        num += 1;
        // Добавление на полотно добавляемую текстуру
        texture_atlas_builder.add_texture(Some(id), texture);
    }

    // Финальная сборка полотна в layout и цельную текстуру
    let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
    // Добавление текстуры в handle
    let texture = textures.add(texture);

    // Обновление настройки выборки в атласе текстур
    let image = textures.get_mut(&texture).unwrap();
    // Применение обработки к текстурам
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture, textures_ids)
}

// ==================================================
const SPRITE_SHEET_W: usize = 16; // размер одного фрагмента по ширине
const SPRITE_SHEET_H: usize = 16; // размер одного фрагмента по высоте

/// Индексирование атласа, путём его разбиения на сетку.
fn load_and_index_atlas(
    folder:     &LoadedFolder,
    padding:    Option<UVec2>,
    sampling:   Option<ImageSampler>,
    textures:   &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>, HashMap<String, usize>) {
    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().padding(padding.unwrap_or_default());

    let mut textures_ids: HashMap<String, usize> = HashMap::new();
    let mut current_index = 0; // Текущий индекс

    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );

            continue;
        };

        // Получение имени загружаемого файла, чтобы использовать это как ключь-имя в hash-таблице
        if let Some(path) = handle.path() {
            if let Some(file_name) = path.to_string().as_str().split('/').last() {
                let file_fmt = Path::new(file_name).file_stem().unwrap().to_string_lossy();
                info!("Loaded resource | {}", file_fmt);
                textures_ids.insert(file_fmt.to_string(), current_index);
            } else {
                warn!("[Error] - An error occurred while reading the file name!")
            }
        } else {
            warn!("[Error] - An error occurred while reading the file path!")
        }

        current_index += 1;
        // Добавление на полотно добавляемую текстуру
        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (_, texture) = texture_atlas_builder.finish().unwrap();
    let texture = textures.add(texture);

    // Обновление настройки выборки в атласе текстур
    let image = textures.get_mut(&texture).unwrap();
        image.sampler = sampling.unwrap_or_default();

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(16. as f32, 16. as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );

    (layout, texture, textures_ids)
}

/*
    Наброски заполнения текстур в атлас и сохранение их id
    1. Поиск текстур в директории
    2. (Текстура найдена) Проверка её расширения и размера текстуры.
    2.1 Если текстура больше стандарта (пусть будет 64 на 64), то для таких будет создан отдельный атлас с с размером специально для них
    3. Установка в атлас текстуры. паралельно получая его id и имя файла, занося в hash таблицу

    1. По ключу имени или id поиск

    ==================================================

    Разные подходы к загрузке директорий.
    Например:
    Entities, т.е. мобы и активные сущности, которые могут активно перемещаться по миру, будут иметь 8-ми сторонную текстуру, а она в свою очередь
    будет представлять собой текстуру 3x3, где по сторонам направления, а в центре непрогруженный вид существа, показывая что он не загружен и виден только силуэт
    В данном случае, будет другой подход к загрузке, не потоковый сборщик текстур в атлас, а уже готовый атлас разбивать на сетку для получения id спрайтов
*/
