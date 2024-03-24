// #![allow(unused)] // Удалить потом
// use bevy::prelude::*;

// use crate::AppState;

// pub struct TileMapPlugin;

// impl Plugin for TileMapPlugin {
//     fn build(&self, app: &mut App) {}
// }

// #[derive(Component)]
// pub struct TileMap {
//     grid_size: i32,
//     tile_size: i32,
// }

// impl TileMap {}

#![allow(unused_imports)]
use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        entity::Entity,
        event::{Event, EventReader},
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    gizmos::{config::GizmoConfig, AppGizmoBuilder},
    math::{IVec2, UVec2, Vec2},
    render::render_resource::FilterMode,
    utils::HashSet,
    window::{Window, WindowPlugin},
    DefaultPlugins,
    prelude::*
};

use bevy_entitiles::{
    debug::CameraAabbScale,
    render::cull::FrustumCulling,
    serializing::{
        chunk::{
            load::{ChunkLoadCache, ChunkLoadConfig},
            save::{self, ChunkSaveCache, ChunkSaveConfig},
        },
        map::TilemapLayer,
    },
    tilemap::{
        buffers::TileBuilderBuffer,
        bundles::StandardTilemapBundle,
        chunking::camera::{CameraChunkUpdater, CameraChunkUpdation},
        map::{
            TileRenderSize, TilemapName, TilemapRotation, TilemapSlotSize, TilemapStorage,
            TilemapTexture, TilemapTextureDescriptor, TilemapType,
        },
        physics::{PhysicsTile, PhysicsTilemap},
        tile::{TileBuilder, TileLayer},
    },
    EntiTilesPlugin,
};

use bevy_xpbd_2d::plugins::{debug::PhysicsGizmos, PhysicsDebugPlugin, PhysicsPlugins};

use crate::AppState;
// use helpers::EntiTilesHelpersPlugin;

// mod helpers;
pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EntiTilesPlugin, 
                PhysicsPlugins::default(), 
                PhysicsDebugPlugin::default(),
            ))
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(Update, on_update.run_if(in_state(AppState::Game)))
            .insert_resource(ChunkSaveConfig {
                path: "generated/chunk_unloading".to_string(),
                chunks_per_frame: 1,
            })
            .insert_resource(ChunkLoadConfig {
                path: "generated/chunk_unloading".to_string(),
                chunks_per_frame: 1,
            })
            .insert_resource(FrustumCulling(false))
            .insert_resource(CameraAabbScale(Vec2::splat(0.3)))
            .insert_gizmo_group(PhysicsGizmos::all(), GizmoConfig::default());
    }
}

#[derive(Event, Debug, Clone, Copy)]
struct GenerateChunk(IVec2);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let entity = commands.spawn_empty().id();
    let mut tilemap = StandardTilemapBundle {
        name: TilemapName("laggy_map".to_string()),
        tile_render_size: TileRenderSize(Vec2::new(16., 16.)),
        slot_size: TilemapSlotSize(Vec2::new(16., 16.)),
        ty: TilemapType::Square,
        storage: TilemapStorage::new(16, entity),
        texture: TilemapTexture::new(
            asset_server.load("test_square.png"),
            TilemapTextureDescriptor::new(
                UVec2 { x: 32, y: 32 },
                UVec2 { x: 16, y: 16 },
                FilterMode::Nearest,
            ),
            TilemapRotation::None,
        ),
        ..Default::default()
    };

    tilemap.storage.reserve_many(
        (-7..=6)
            .into_iter()
            .flat_map(move |x| (-7..=6).into_iter().map(move |y| IVec2 { x, y })),
    );

    tilemap.storage.fill_rect(
        &mut commands,
        bevy_entitiles::math::TileArea::new(IVec2 { x: -100, y: -100 }, UVec2 { x: 200, y: 200 }),
        TileBuilder::new().with_layer(0, TileLayer::no_flip(0)),
    );

    let mut physics_tilemap = PhysicsTilemap::new_with_chunk_size(16);
    physics_tilemap.fill_rect_custom(
        bevy_entitiles::math::TileArea::new(IVec2 { x: -100, y: -100 }, UVec2 { x: 200, y: 200 }),
        |_| {
            if rand::random::<u32>() % 10 == 0 {
                Some(PhysicsTile {
                    rigid_body: true,
                    friction: Some(0.2),
                })
            } else {
                None
            }
        },
        false,
    );
    commands.entity(entity).insert(physics_tilemap);

    commands.entity(entity).insert(tilemap);
}

fn on_update(
    mut commands: Commands,
    mut ev: EventReader<CameraChunkUpdation>,
    tilemap: Query<Entity, With<TilemapStorage>>,
    mut load_cache: ResMut<ChunkLoadCache>,
    mut save_cache: ResMut<ChunkSaveCache>,
) {
    let tilemap = tilemap.single();
    let mut to_load = Vec::new();
    let mut to_unload = Vec::new();

    ev.read().for_each(|e| match e {
        CameraChunkUpdation::Entered(_, chunk) => to_load.push(*chunk),
        CameraChunkUpdation::Left(_, chunk) => to_unload.push((*chunk, true)),
    });

    // You can actually do everything you want
    // This case we load/save the chunk

    if !to_load.is_empty() {
        load_cache.schedule_many(
            &mut commands,
            tilemap,
            TilemapLayer::COLOR | TilemapLayer::PHYSICS,
            to_load.into_iter(),
        );
    }

    if !to_unload.is_empty() {
        save_cache.schedule_many(
            &mut commands,
            tilemap,
            TilemapLayer::COLOR | TilemapLayer::PHYSICS,
            to_unload.into_iter(),
        );
    }
}

// === Helpers

// use std::{fmt::Debug, time::Duration};

// use bevy::{
//     prelude::{default, Color, Commands, IntoSystemConfigs, Plugin, Startup, TextBundle, Update},
//     text::{TextSection, TextStyle},
//     time::common_conditions::on_real_timer,
// };
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// // use bevy_inspector_egui::quick::WorldInspectorPlugin;

// use crate::helpers::camera_movement::camera_control;

// use self::{
//     camera_movement::CameraControl,
//     common::{debug_info_display, DebugFpsText},
// };

// // pub mod camera_movement;
// // pub mod common;

// pub struct EntiTilesHelpersPlugin {
//     pub inspector: bool,
// }

// impl Default for EntiTilesHelpersPlugin {
//     fn default() -> Self {
//         Self { inspector: true }
//     }
// }

// impl Plugin for EntiTilesHelpersPlugin {
//     fn build(&self, app: &mut bevy::prelude::App) {
//         app.add_systems(Startup, debug_startup).add_systems(
//             Update,
//             (
//                 camera_control,
//                 debug_info_display.run_if(on_real_timer(Duration::from_millis(100))),
//             ),
//         );

//         app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);

//         if self.inspector {
//             app.add_plugins(WorldInspectorPlugin::default());
//         }

//         app.init_resource::<CameraControl>();
//     }

//     fn finish(&self, _app: &mut bevy::prelude::App) {
//         // print_render_graph(_app);
//     }
// }

// pub fn debug_startup(mut commands: Commands) {
//     commands.spawn((
//         DebugFpsText,
//         TextBundle::from_sections([
//             TextSection::new(
//                 "FPS: ",
//                 TextStyle {
//                     font_size: 32.,
//                     color: Color::WHITE,
//                     ..default()
//                 },
//             ),
//             TextSection::new(
//                 "",
//                 TextStyle {
//                     font_size: 32.,
//                     color: Color::WHITE,
//                     ..default()
//                 },
//             ),
//         ]),
//     ));
// }

// pub fn validate_heap<K: PartialOrd + Debug, V: Debug>(tree: &Vec<Option<(K, V)>>, asc: bool) {
//     for i in 1..tree.len() {
//         if let Some((k1, _)) = &tree[i] {
//             let left = i * 2;
//             let right = i * 2 + 1;
//             if left < tree.len() {
//                 if let Some((k2, _)) = &tree[left] {
//                     if asc {
//                         assert!(k1 <= k2, "heap validation failed at index {}", i);
//                     } else {
//                         assert!(k1 >= k2, "heap validation failed at index {}", i);
//                     }
//                 }
//             }
//             if right < tree.len() {
//                 if let Some((k2, _)) = &tree[right] {
//                     if asc {
//                         assert!(k1 <= k2, "heap validation failed at index {}", i);
//                     } else {
//                         assert!(k1 >= k2, "heap validation failed at index {}", i);
//                     }
//                 }
//             }
//         }
//     }
//     println!("heap validated √");
// }