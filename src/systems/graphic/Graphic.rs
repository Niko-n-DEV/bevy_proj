//#![allow(unused)]
use std::collections::HashMap;

use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

use crate::core::{AppState, graphic::Atlas::TestTextureAtlas};

/// Ресурс хранящий в себе загружаемую папку ресурсов
#[derive(Resource, Default)]
struct ResourceFolder(Handle<LoadedFolder>);

pub struct Graphic;

impl Plugin for Graphic {
    fn build(&self, app: &mut App) {
        app
            // Взятие ресурсов из assets
            .add_systems(OnEnter(AppState::ResourceCheck), load_resource_folder)
            // Ну, атлас, да
            .insert_resource(TestTextureAtlas::default())
            // Проверка ресурсов на зависимости (Непонятно как оно точно работает)
            .add_systems(Update,check_textures.run_if(in_state(AppState::ResourceCheck)))
            .add_systems(OnEnter(AppState::ResourceLoading), setup_ex)
            ;
    }
}

/// функция для загрузки ресурсов из определённой папки, по умолчанию эта папка - assets, и всё его содержимое
fn load_resource_folder(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ResourceFolder(asset_server.load_folder("")));
}

/// Проверка чего-то с каждым обновлением
fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    resource_folder: Res<ResourceFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&resource_folder.0) {
            next_state.set(AppState::ResourceLoading);
        }
    }
}

fn setup_ex(
    // mut commands: Commands,
    // asset_server: Res<AssetServer>,
    resource_handle: Res<ResourceFolder>,
    mut handle: ResMut<TestTextureAtlas>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    let loaded_folder = loaded_folders.get(&resource_handle.0).unwrap();

    let (texture_atlas_nearest, nearest_texture, _hash) = create_texture_atlas(
        &loaded_folder,
        None,
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let atlas_nearest_handle = texture_atlases.add(texture_atlas_nearest);

    handle.layout = Some(atlas_nearest_handle);
    handle.image = Some(nearest_texture);

    next_state.set(AppState::MainMenu);
}

/// Создание атласа текстур с заданными настройками заполнения и выборки из отдельных спрайтов в данной папке
fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (
    TextureAtlasLayout,
    Handle<Image>,
    HashMap<String, AssetId<Image>>,
) {
    let mut textures_ids: HashMap<String, AssetId<Image>> = HashMap::new();
    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().padding(padding.unwrap_or_default());

    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };
        textures_ids.insert("Test".to_string(), id.clone());
        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
    let texture = textures.add(texture);

    // Обновление настройки выборки в атласе текстур
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture, textures_ids)
}

/// Создание и установка спрайта и атласа
fn sprite_from_atlas(
    commands: &mut Commands,
    sprite_index: usize,
    atlas_handle: Handle<TextureAtlasLayout>,
    texture: Handle<Image>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform { ..default() },
            texture,
            ..default()
        },
        TextureAtlas {
            layout: atlas_handle,
            index: sprite_index,
        },
    ));
}

/*
    Наброски заполнения текстур в атлас и сохранение их id
    1. Поиск текстур в директории
    2. (Текстура найдена) Проверка её расширения и размера текстуры.
    2.1 Если текстура больше стандарта (пусть будет 64 на 64), то для таких будет создан отдельный атлас с с размером специально для них
    3. Установка в атлас текстуры. паралельно получая его id и имя файла, занося в hash таблицу

    1. По ключу имени или id поиск
*/
