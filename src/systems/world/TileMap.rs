use bevy::{
    app::{PluginGroup, Update},
    ecs::{query::With, system::Query},
    input::{keyboard::KeyCode, ButtonInput},
    math::IVec2,
    prelude::*,
    render::{color::Color, render_resource::FilterMode, view::Visibility},
};
use bevy_entitiles::{
    math::TileArea,
    tilemap::{
        bundles::StandardTilemapBundle,
        map::{
            TileRenderSize, TilemapName, TilemapRotation, TilemapSlotSize, TilemapStorage,
            TilemapTexture, TilemapTextureDescriptor, TilemapTransform, TilemapType,
        },
        tile::{LayerUpdater, TileBuilder, TileLayer, TileLayerPosition, TileUpdater},
    }
};

#[derive(Component)]
pub struct TileM; 


pub fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {

    let entity = commands.spawn_empty().id();

    let mut tilemap = StandardTilemapBundle {
        name: TilemapName("test_map".to_string()),
        tile_render_size: TileRenderSize(Vec2 { x: 16., y: 16. }),
        slot_size: TilemapSlotSize(Vec2 { x: 16., y: 16. }),
        ty: TilemapType::Square,
        storage: TilemapStorage::new(16, entity),
        texture: TilemapTexture::new(
            assets_server.load("core/textures/terrain/tiles_test.png"),
            TilemapTextureDescriptor::new(
                UVec2 { x: 64, y: 64 },         // Это размер атласа
                UVec2 { x: 16, y: 16 },    // Это размер клетки
                FilterMode::Nearest,
            ),
            TilemapRotation::None,
        ),
        transform: TilemapTransform {
            z_index: -1.0,
            ..default()
        },
        ..Default::default()
    };

    // for x in 0..10 {
    //     for y in 0..10 {
    //         tilemap.storage.set(
    //             &mut commands,
    //             IVec2 { x, y },
    //             TileBuilder::new().with_layer(1, TileLayer::no_flip(7)),
    //         );
    //     }
    // }

    // tilemap.storage.set(
    //     &mut commands,
    //     IVec2 { x: 0, y: 0 },
    //     TileBuilder::new().with_layer(1, TileLayer::no_flip(2)),
    // );

    // tilemap.storage.set(
    //     &mut commands,
    //     IVec2 { x: -1, y: -1 },
    //     TileBuilder::new().with_layer(1, TileLayer::no_flip(1)),
    // );

    commands.entity(entity).insert((tilemap, TileM));
}

#[derive(Event)]
pub struct LoadChunkPos(pub IVec2);

pub fn fill_chunk(
    mut commands: Commands,
    mut tilem: Query<(Entity, &mut TilemapStorage), With<TileM>>,
    mut tilem_pos: EventReader<LoadChunkPos>
) {
    if tilem_pos.is_empty() {
        return;
    }

    let (tilemap, mut storage) = tilem.single_mut();

    // for t in tilem.iter() {
    //     println!("{:?}", t);
    // }

    for tilem_pos in tilem_pos.read() {
        for x in tilem_pos.0.x*16..tilem_pos.0.x*16+16 {
            for y in tilem_pos.0.y*16..tilem_pos.0.y*16+16 {
                if x == 1 || y == 1 {
                    storage.set(
                        &mut commands,
                        IVec2 { x, y },
                        TileBuilder::new().with_layer(1, TileLayer::no_flip(1)),
                    );
                } else {
                    storage.set(
                        &mut commands,
                        IVec2 { x, y },
                        TileBuilder::new().with_layer(1, TileLayer::no_flip(6)),
                    );
                }
                
            }
        }
    }
}

#[derive(Event)]
pub struct DischargeChunkPos(pub IVec2);

pub fn clear_chunk(
    mut commands: Commands,
    mut tilem: Query<(Entity, &mut TilemapStorage), With<TileM>>,
    mut tilem_pos: EventReader<DischargeChunkPos>
) {
    if tilem_pos.is_empty() {
        return;
    }

    let (tilemap, mut storage) = tilem.single_mut();

    for tilem_pos in tilem_pos.read() {
        for x in tilem_pos.0.x*16..tilem_pos.0.x*16+16 {
            for y in tilem_pos.0.y*16..tilem_pos.0.y*16+16 {
                storage.remove(
                    &mut commands,
                    IVec2 { x, y },
                );
            }
        }
    }
}

pub fn toggle(
    mut tilemaps_query: Query<&mut Visibility, With<TilemapStorage>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for mut visibility in tilemaps_query.iter_mut() {
            *visibility = match *visibility {
                Visibility::Inherited => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                Visibility::Visible => Visibility::Hidden,
            }
        }
    }
}