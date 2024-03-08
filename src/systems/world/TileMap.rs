#![allow(unused)] // Удалить потом
use bevy::{prelude::*, render::render_resource::FilterMode};
use bevy_entitiles::{
    math::TileArea,
    tilemap::{
        bundles::{StandardPureColorTilemapBundle, StandardTilemapBundle},
        map::{
            TileRenderSize, TilemapName, TilemapRotation, TilemapSlotSize, TilemapStorage,
            TilemapTexture, TilemapTextureDescriptor, TilemapTransform, TilemapType,
        },
        tile::{LayerUpdater, TileBuilder, TileLayer, TileLayerPosition, TileUpdater},
    },
    EntiTilesPlugin,
};


pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

fn setup(
    mut commands: Commands, 
    assets_server: Res<AssetServer>
) {
    let entity = commands.spawn_empty().id();

    let mut tilemap = StandardTilemapBundle {
        name: TilemapName("test_map".to_string()),
        tile_render_size: TileRenderSize(Vec2 { x: 16., y: 16. }),
        slot_size: TilemapSlotSize(Vec2 { x: 16., y: 16. }),
        ty: TilemapType::Square,
        storage: TilemapStorage::new(16, entity),
        texture: TilemapTexture::new(
            assets_server.load("test_square.png"),
            TilemapTextureDescriptor::new(
                UVec2 { x: 32, y: 32 },
                UVec2 { x: 16, y: 16 },
                FilterMode::Nearest,
            ),
            TilemapRotation::None,
        ),
        ..Default::default()
    };
}