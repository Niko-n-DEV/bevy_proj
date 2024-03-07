#![allow(unused)]

use crate::core::AppState;
use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

#[derive(Resource, Default)]
struct ResourceFolder(Handle<LoadedFolder>);

fn load_resource_folder(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ResourceFolder(asset_server.load_folder("assets")));
}

fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    resource_folder: Res<ResourceFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&resource_folder.0) {
            next_state.set(AppState::Finished);
        }
    }
}

fn setup(
    mut commands: Commands,
    resource_handle: Res<ResourceFolder>,
    asser_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let loaded_folder = loaded_folders.get(&resource_handle.0).unwrap();
    // // Создание атлосов с различной выборкой
    // let (texture_atlas_linear, linear_texture) = create_texture_atlas(
    //     loaded_folder,
    //     None,
    //     Some(ImageSampler::linear()),
    //     &mut textures,
    // );

    // let atlas_linear_handle = texture_atlases.add(texture_atlas_linear.clone());

    let (texture_atlas_nearest, nearest_texture) = create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let atlas_nearest_handle = texture_atlases.add(texture_atlas_nearest);

    let (texture_atlas_nearest_padded, nearest_padded_texture) = create_texture_atlas(
        loaded_folder,
        Some(UVec2::new(6, 6)),
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let atlas_nearest_padded_handle = texture_atlases.add(texture_atlas_nearest_padded);

    // draw unpadded texture atlas
    commands.spawn(SpriteBundle {
        texture: nearest_texture.clone(),
        transform: Transform {
            translation: Vec3::new(-250.0, -130.0, 0.0),
            scale: Vec3::splat(0.8),
            ..default()
        },
        ..default()
    });

    // draw padded texture atlas
    commands.spawn(SpriteBundle {
        texture: nearest_padded_texture.clone(),
        transform: Transform {
            translation: Vec3::new(250.0, -130.0, 0.0),
            scale: Vec3::splat(0.8),
            ..default()
        },
        ..default()
    });
}

/// Создание атласа текстур с заданными настройками заполнения и выборки из отдельных спрайтов в данной папке
fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>) {
    // Постройка атласа используя индивидуальные спрайты
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

        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
    let texture = textures.add(texture);

    // Обновление настройки выборки в атласе текстур
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture)
}
