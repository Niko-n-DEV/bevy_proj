use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use std::{collections::HashMap, marker::PhantomData};

// use bevy_entitiles::EntiTilesPlugin;

use crate::core::{
    entities::{
        ai::AiPlugin,
        EntitySystem::EntitySystem,
    },
    resource::graphic::Atlas::AtlasRes,
    world::{
        chunk::Chunk::Chunk, 
        // TileMap::{
        //     self, 
        //     DischargeChunkPos, 
        //     LoadChunkPos
        // }, 
        WorldTaskManager
    }, 
    AppState, 
    ContainerSystem::ContainerPlugin, 
    Entity::{
        EntitySpawn,
        spawn_entity
    },
    Item::item_plugin,
    ItemType::ItemType, 
    Object::{
        ObjectSpawn,
        spawn_object
    },
    Settings::Settings, 
    UserSystem::UserControl,
    PlayerSystem::PlayerPlugin,
    interact::Damage::DamageSystem
};

use super::Grid::Grid;

pub struct WorldSystem;

impl Plugin for WorldSystem {
    fn build(&self, app: &mut App) {
        app
            // Init Plugins
            // .add_plugins(EntiTilesPlugin)
            .add_plugins((
                // Физика
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0),
                RapierDebugRenderPlugin {
                    enabled: !true,
                    ..default()
                }
            ))
            .add_plugins(
                (
                    EntitySystem,   // Инициализация плагина, отвечающего за работу всех entity
                    AiPlugin,
                    PlayerPlugin,   // Инициализация плагина, отвечающего за работу управления entity-player
                    DamageSystem,
                    ContainerPlugin::<ItemType> {
                        phantom: PhantomData {}
                    },
                )
            )
            .add_plugins(item_plugin)
            // Init Event
            .add_event::<ObjectSpawn>()
            .add_event::<EntitySpawn>()
            // Init Resource
            .init_resource::<WorldRes>()
            .init_resource::<Chunk>()
            // Init Systems
            .add_systems(
                OnEnter(AppState::LoadingInGame),
                (
                    WorldTaskManager::load_data,
                    // TileMap::setup
                )
            )
            .add_systems(OnEnter(AppState::Game), 
            (
                Self::setup, 
                Self::init_world.after(Self::setup)
            ))
            .add_systems(FixedUpdate, 
                (
                    spawn_object,
                    spawn_entity
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(
                Update,
                (
                    Self::load_chunk_around,
                    Self::update_grid,
                    // TileMap::toggle,
                    // TileMap::fill_chunk,
                    // TileMap::clear_chunk
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(OnExit(AppState::Game), (
                WorldTaskManager::despawn_entities,
                // WorldTaskManager::despawn_object,
                Self::upload_data,
                WorldTaskManager::despawn_items,
                WorldTaskManager::despawn_terrain
            ))
        ;

    }
}

impl WorldSystem {

    fn setup(
        mut commands:       Commands, 
        //     asset_server:   Res<AssetServer>,
            settings:       Res<Settings>,
        mut world:          ResMut<WorldRes>,
        mut physics:        ResMut<RapierConfiguration>,
    ) {
        world.player_render_distance = settings.rendering_distance;

        commands.insert_resource(Grid::new(settings.rendering_distance));

        physics.gravity = Vec2::ZERO;
    }

    /// Функция инициализации мира, где будет производиться загрузка всех компонентов мира.
    /// 
    /// Установка как задачи процессы загрузки ресурсов и прогрузки отдельных комплексных компонентов.
    fn init_world(
        // mut commands:       Commands,
        //     atlas:          Res<AtlasRes>,
        //     handle:         Res<TestTextureAtlas>,
        //     handle_dir:     Res<DirectionAtlas>,
        // mut register:       ResMut<Registry>,
        // mut obj_event:      EventWriter<ObjectSpawn>,
        // mut item_event:     EventWriter<ItemSpawn>,
        mut entity_event:   EventWriter<EntitySpawn>,
    ) {
        /*
            Тут будет непосредственно инициализация мира, где будет размещение игровой сетки, основных его компонентов и сущностей.
            Установка синхронно с процессом загрузки ресурсов из файла.
        */

        entity_event.send(EntitySpawn("human".to_string(), Vec2::splat(16.0)));
    }

    fn update_grid(
        mut commands:       Commands,
        mut grid:           ResMut<Grid>,
            atlas:          Res<AtlasRes>,
            player_query:   Query<&Transform, With<UserControl>>,
    ) {
        if player_query.is_empty() {
            return;
        }
        
        if let Ok(player_transform) = player_query.get_single() {
            let player_pos = player_transform.translation.truncate().as_ivec2();
            grid.update_chunks(&mut commands, &atlas, player_pos);
        }
    }

    fn upload_data(
        mut commands:   Commands,
        mut grid:       ResMut<Grid>
    ) {
        grid.upload_all(&mut commands);
    }

    /// Функция для инициализации загрузки чанков вокруг игрока в пределах установленной прогрузки.
    fn load_chunk_around(
        // mut commands:       Commands,
        //    asset_server:   Res<AssetServer>,
        mut worldres:       ResMut<WorldRes>,
        //    handle:         Res<TestTextureAtlas>,
            player_query:   Query<(&mut Transform, &mut UserControl)>,
        // mut chunk_load:     EventWriter<LoadChunkPos>,
        // mut chunk_upload:   EventWriter<DischargeChunkPos>
    ) {
        if player_query.is_empty() || true {
            return;
        }
        /*
            По сути потоковая функция, которая будет прогружать территорию
            Небольшой черновик работы функции.
            Отправляется от сущности "Игрок", вычисляется где он находиться и прогружает во круг чанки, если они находяться в зоне прогрузки.
            Изберательная прогрузка: У игрока есть некая зона, в пределах которой не прогружается далее территория, если зайти за предел радиуса допуска, то происходит загрузка.
            Та некая зона появляется, если стоять на месте некоторое время (зона афк))
            При процессе загрузки чанков, сначала происходит проверка [check_exists_chunk], есть ли чанк, если нету происходит генерация [generate_chunk], если же чанк был найден, происходит загрузка чанка [load_chunk], создавая экземпляр
            сцены в res//, с именем, который является его номером по координатной сетке (0_0 | 0_1 | 1_0 и т.д.). Сам чанк, сохраняется и выгружается из файла, который представляет собой своеобразный сжатый "архив",
            хронящий информацию о 32x32 чанках по координатной сетке, т.е. есть "архив" в нём 32x32 чанка, он занимает положение в мире 0_0 и т.д.
        */

        for (transform, _player) in &player_query {
            let player_translation = transform.translation.truncate().as_ivec2();
            worldres.player_chunk_position = Self::get_current_chunk(player_translation)
        }

        if worldres.player_chunk_position == worldres.player_chunk_last_position
            && worldres.first_launch
        {
            worldres.player_chunk_last_position = IVec2::ZERO;
            worldres.first_launch = false
        }

        if worldres.player_chunk_position != worldres.player_chunk_last_position {
            worldres.player_chunk_last_position = worldres.player_chunk_position;

            let (player_chunk_x, player_chunk_y) = (
                worldres.player_chunk_position.x,
                worldres.player_chunk_position.y,
            );

            // Нужна более чательная проработка
            let mut loaded_chunks_new: Vec<IVec2> = Vec::new();
            for x in (player_chunk_x - worldres.player_render_distance)
                ..=(player_chunk_x + worldres.player_render_distance)
            {
                for y in (player_chunk_y - worldres.player_render_distance)
                    ..=(player_chunk_y + worldres.player_render_distance)
                {
                    let chunk_pos = IVec2::new(x, y);
                    loaded_chunks_new.push(chunk_pos);
                }
            }

            let mut chunks_to_discharge: Vec<IVec2> = Vec::new();
            let mut chunks_to_discharge_test: Vec<IVec2> = Vec::new();
            // Проверяет в chunk, есть ли чанки, которые не входят в радиус прогрузки, чтобы их выгрузить
            for pos in &worldres.chunk {
                if !loaded_chunks_new.contains(pos) {
                    chunks_to_discharge.push(*pos);
                }
            }
            // test
            for (pos, _) in &worldres.chunks {
                if !loaded_chunks_new.contains(pos) {
                    chunks_to_discharge_test.push(*pos);
                }
            }

            // Проверяет, есть ли чанки в списке на прогрузку, которые ещё не загружены, чтобы их загрузить.
            let mut chunks_to_upload: Vec<IVec2> = Vec::new();
            let mut chunks_to_upload_test: Vec<IVec2> = Vec::new();
            for chunk in &loaded_chunks_new {
                if !worldres.chunk.contains(chunk) {
                    chunks_to_upload.push(*chunk);
                }
            }
            // test
            for chunk in &loaded_chunks_new {
                if !worldres.chunks.contains_key(chunk) {
                    chunks_to_upload_test.push(*chunk);
                }
            }

            for chunk in chunks_to_discharge {
                // chunk_upload.send(DischargeChunkPos(chunk));
                let chunk_list_len = worldres.chunk.len();
                for index in 0..chunk_list_len-1 {
                    if &chunk == worldres.chunk.get(index).unwrap() {
                        worldres.chunk.remove(index);
                    }
                }
            }

            for chunk in chunks_to_upload {
                // chunk_load.send(LoadChunkPos(chunk));
                worldres.chunk.push(chunk);
            }

            // // test
            // for chunk in chunks_to_upload_test {
            // //    Self::create_chunk(&mut commands, &asset_server, &mut worldres, &handle, chunk);
            // }
            // for chunk in chunks_to_discharge_test {
            // //    Self::despawn_chunk(&mut commands, &mut worldres, chunk);
            // }
        }
    }

    // ==============================
    // TEST
    // ==============================
    // #[allow(unused)]
    // fn create_chunk(
    // //    gizmos:         &mut Gizmos,
    //     commands:       &mut Commands,
    //     asset_server:   &Res<AssetServer>,
    //     world_res:      &mut ResMut<WorldRes>,
    //     handle:         &Res<TestTextureAtlas>,
    //     pos:            IVec2
    // ) { // -> Entity {
    // //    gizmos.rect_2d(Vec2::new(pos.x as f32 * 256.0, pos.y as f32 * 256.0), 0.0, Vec2::splat(16.0), Color::YELLOW_GREEN);
    //     // let chunk = commands
    //     //     .spawn(SpriteSheetBundle {
    //     //         sprite: Sprite {
    //     //             anchor: bevy::sprite::Anchor::BottomLeft,
    //     //             ..default()
    //     //         },
    //     //         texture: handle.image.clone().unwrap(),
    //     //         atlas: TextureAtlas {
    //     //             layout: handle.layout.clone().unwrap(),
    //     //             index: TestTextureAtlas::get_index("dirt", &handle),
    //     //         },
    //     //         transform: Transform {
    //     //             translation: Vec3::new(pos.x as f32 * 256.0, pos.y as f32 * 256.0, -1.0),
    //     //             scale: Vec3::new(16.0, 16.0, 0.0),
    //     //             ..default()
    //     //         },
    //     //         ..default()
    //     //     })
    //     //     .insert(Name::new(format!("{pos}_chunk")))
    //     //     .id();
    //     // world_res.chunks.insert(pos, chunk);
    //     // chunk
    // }

    // #[allow(unused)]
    // fn despawn_chunk(
    //     commands: &mut Commands,
    //     world: &mut ResMut<WorldRes>,
    //     //chunk: &Chunk,
    //     pos: IVec2,
    // ) {
    //     if let Some(entity) = world.chunks.remove(&pos) {
    //         commands.entity(entity).despawn();
    //     } else {
    //         println!("despawn failed")
    //     }
    // }

    /// Функция для определения точных координат чанка
    ///
    /// Определяется по данной позиции и делением на общий размер чанка
    pub fn get_current_chunk(input_var: IVec2) -> IVec2 {
        let result = Self::get_format_current_chunk(input_var);
        result
    }

    /// Функция для форматирования значения чанков по координатной системе
    pub fn get_format_current_chunk(input_var: IVec2) -> IVec2 {
        let mut chunk_x = input_var.x / 256;
        let mut chunk_y = input_var.y / 256;
        if input_var.x < 0 {
            chunk_x -= 1;
        }
        if input_var.y < 0 {
            chunk_y -= 1;
        }
        IVec2::new(chunk_x, chunk_y)
    }

    /// Функция для определения точных координат тайла в чанке
    ///
    /// Определяется по данной позиции и позиции чанка, делением на общий размер одного тайла
    pub fn get_currect_chunk_tile(input_var: IVec2) -> IVec2 {
        let tile_size = 16;
        let result: IVec2 = IVec2::new(
            if input_var.x >= 0 {
                input_var.x / tile_size
            } else {
                (input_var.x - tile_size + 1) / tile_size
            },
            if input_var.y >= 0 {
                input_var.y / tile_size
            } else {
                (input_var.y - tile_size + 1) / tile_size
            }
        );
        result
    }

    /// Функция для определения точных координат суб-тайлов в чанке
    pub fn get_currect_chunk_subtile(input_var: IVec2) -> IVec2 {
        let sub_size = 8;
        let result: IVec2 = IVec2::new(
            if input_var.x >= 0 {
                input_var.x / sub_size
            } else {
                (input_var.x - sub_size + 1) / sub_size
            },
            if input_var.y >= 0 {
                input_var.y / sub_size
            } else {
                (input_var.y - sub_size + 1) / sub_size
            }
        );
        result
    }

    /// Функция для определения координат тайла в пределах чанка
    ///
    /// Определяется по данной позиции и определением координат в пределах одного чанка, где отсчёт координат начинается с верхнего левого угла чанка.
    #[allow(unused)]
    pub fn get_local_tile_chunk(input_var: IVec2) {}
}

#[derive(Component, Resource)]
pub struct WorldRes {
    player_render_distance: i32,
    player_chunk_position: IVec2,
    player_chunk_last_position: IVec2,
    first_launch: bool,
    // chunk_size_t: i32,

    chunks: HashMap<IVec2, Entity>,
    chunk: Vec<IVec2>
}

impl Default for WorldRes {
    fn default() -> Self {
        Self {
            player_render_distance: 3,
            player_chunk_position: IVec2 { x: 0, y: 0 },
            player_chunk_last_position: IVec2 { x: 0, y: 0 },
            first_launch: true,
            // chunk_size_t: 256,

            chunks: HashMap::new(),
            chunk: Vec::new()
        }
    }
}

//
//
//

#[allow(unused)]
#[derive(Resource)]
pub struct WorldInfo {
    pub seed:           u64,

    pub name:           String,

    pub daytime:        f32,

    // seconds a day time long
    pub daytime_length: f32,

    // seconds
    pub time_inhabited: f32,

        time_created:   u64,
        time_modified:  u64,

        tick_timer:     Timer,

    pub is_paused:      bool,
    pub paused_steps:   i32,
    // pub is_manipulating: bool, 
}

impl Default for WorldInfo {
    fn default() -> Self {
        WorldInfo {
            seed: 0,
            name: "None Name".into(),
            daytime: 0.15,
            daytime_length: 60. * 24.,

            time_inhabited: 0.,
            time_created: 0,
            time_modified: 0,

            tick_timer: Timer::new(bevy::utils::Duration::from_secs_f32(1. / 20.), TimerMode::Repeating),

            is_paused: false,
            paused_steps: 0,
            // is_manipulating: true,
        }
    }
}