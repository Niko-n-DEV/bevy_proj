#![allow(unused)]
use std::collections::HashMap;

use crate::core::AppState;
use bevy::{asset::LoadedFolder, prelude::*, reflect::Enum, render::texture::ImageSampler};

#[derive(Resource, Default)]
struct ResourceFolder(Handle<LoadedFolder>);

fn load_resource_folder(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ResourceFolder(asset_server.load_folder("assets")));
}

#[derive(Resource)]
pub struct Atlas {
    texture_id: HashMap<String, AssetId<Image>>
}

impl Atlas {
    pub fn new(texture_id: HashMap<String, AssetId<Image>>) -> Self {
        Atlas { 
            texture_id,
        }
    }

    // pub fn get_texture(
    //     mut command: Commands,
    //     atlas: ResMut<Atlas>,
    //     asset_server: Res<AssetServer>,
    //     base: &Self,
    //     name: &str
    // ) {
    //     if let Some(id) = base.texture_id.get(name) {
    //         let temp = TextureAtlas {
    //             layout: &atlas.atlas_layout.,
    //             index: id.variant_index()
    //         };
    //     }
    // }
}

pub struct Graphic;

impl Plugin for Graphic {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::ResourceLoading), load_resource_folder)
            .add_systems(Update, check_textures.run_if(in_state(AppState::ResourceLoading)))
            .add_systems(OnExit(AppState::ResourceLoading), setup_ex);
    }
}



fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    resource_folder: Res<ResourceFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&resource_folder.0) {
            next_state.set(AppState::ResourceCorrect);
        }
    }
}

fn setup_ex(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resource_handle: Res<ResourceFolder>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let loaded_folder = loaded_folders.get(&resource_handle.0).unwrap();
    // Создание атлосов с различной выборкой
    let (texture_atlas, texture, ids ) = create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::linear()),
        &mut textures,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    Atlas::new(ids);
}

// fn setup(
//     mut commands: Commands,
//     resource_handle: Res<ResourceFolder>,
//     asser_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
//     loaded_folders: Res<Assets<LoadedFolder>>,
//     mut textures: ResMut<Assets<Image>>,
// ) {
//     let loaded_folder = loaded_folders.get(&resource_handle.0).unwrap();
//     // // Создание атлосов с различной выборкой
//     // let (texture_atlas_linear, linear_texture) = create_texture_atlas(
//     //     loaded_folder,
//     //     None,
//     //     Some(ImageSampler::linear()),
//     //     &mut textures,
//     // );

//     // let atlas_linear_handle = texture_atlases.add(texture_atlas_linear.clone());

//     let (texture_atlas_nearest, nearest_texture) = create_texture_atlas(
//         loaded_folder,
//         None,
//         Some(ImageSampler::nearest()),
//         &mut textures,
//     );
//     let atlas_nearest_handle = texture_atlases.add(texture_atlas_nearest);

//     let (texture_atlas_nearest_padded, nearest_padded_texture) = create_texture_atlas(
//         loaded_folder,
//         Some(UVec2::new(6, 6)),
//         Some(ImageSampler::nearest()),
//         &mut textures,
//     );
//     let atlas_nearest_padded_handle = texture_atlases.add(texture_atlas_nearest_padded);
// }

/// Создание атласа текстур с заданными настройками заполнения и выборки из отдельных спрайтов в данной папке
fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>, HashMap<String, AssetId<Image>>) {

    let mut textures_ids: HashMap<String, AssetId<Image>> = HashMap::new();
    let mut texture_atlas_builder = TextureAtlasBuilder::default().padding(padding.unwrap_or_default());

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
            transform: Transform {
                ..default()
            },
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