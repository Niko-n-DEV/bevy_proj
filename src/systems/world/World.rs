use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{
        Collider, 
        Velocity, 
        *
    }, 
    rapier::dynamics::RigidBodyDamping
};

use std::collections::HashMap;

use bevy_entitiles::EntiTilesPlugin;

use crate::core::{
    entities::EntitySystem::EnemySpawner,
    resource::{
        graphic::Atlas::{
            AtlasRes,
            DirectionAtlas, 
            TestTextureAtlas
        },
        Registry::Registry,
        SpriteLayer
    },
    world::{
        chunk::Chunk::Chunk, 
        TileMap::{
            self, 
            DischargeChunkPos, 
            LoadChunkPos
        }, WorldTaskManager
    }, 
    AppState, 
    Container::Container, 
    Entity::{
        EntityBase,
        Health,
        Speed,
        Position,
        EntitySpawn
    },
    EntityType::{
        EntityType,
        HumonoidType,
        EntityNeutrality
    },
    Item::{
        ItemSpawn,
        spawn_item
    },
    ItemType::{
        Ammo, 
        Item, 
        ItemType, 
        Pickupable,
        Material
    }, 
    Object::{
        EntityObject,
        ObjectSpawn,
        spawn_object
    },
    Movement::DirectionState,
    Settings::Settings, 
    UserSystem::User,
    PlayerSystem::PlayerAttach, 
    Weapon::GunController, 
};

pub struct WorldSystem;

impl Plugin for WorldSystem {
    fn build(&self, app: &mut App) {
        app
            // Init Plugins
            .add_plugins(EntiTilesPlugin)
            .add_plugins((
                // Физика
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0),
                RapierDebugRenderPlugin {
                    enabled: !true,
                    ..default()
                }
            ))
            // Init Event
            .add_event::<LoadChunkPos>()
            .add_event::<DischargeChunkPos>()
            .add_event::<ItemSpawn>()
            .add_event::<ObjectSpawn>()
            // Init Resource
            .init_resource::<WorldRes>()
            .init_resource::<Chunk>()
            // Init Systems
            .add_systems(
                OnEnter(AppState::LoadingInGame),
                (
                    WorldTaskManager::load_data,
                    TileMap::setup
                )
            )
            .add_systems(OnEnter(AppState::Game), 
            (
                Self::setup, 
                Self::init_world.after(Self::setup)
            ))
            .add_systems(FixedUpdate, 
                (
                    spawn_item,
                    spawn_object
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(
                Update,
                (
                    Self::load_chunk_around,
                    TileMap::toggle,
                    TileMap::fill_chunk,
                    TileMap::clear_chunk
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(OnExit(AppState::Game), (
                WorldTaskManager::despawn_entities,
                WorldTaskManager::despawn_object,
                WorldTaskManager::despawn_terrain
            ))
        ;

    }
}

impl WorldSystem {

    fn setup(
        mut commands:       Commands, 
            asset_server:   Res<AssetServer>,
            settings:       Res<Settings>,
        mut world:          ResMut<WorldRes>,
        mut physics:        ResMut<RapierConfiguration>
    ) {
        world.player_render_distance = settings.rendering_distance;

        physics.gravity = Vec2::ZERO;
    }

    /// Функция инициализации мира, где будет производиться загрузка всех компонентов мира.
    /// 
    /// Установка как задачи процессы загрузки ресурсов и прогрузки отдельных комплексных компонентов.
    fn init_world(
        mut commands:       Commands,
            atlas:          Res<AtlasRes>,
            handle:         Res<TestTextureAtlas>,
            handle_dir:     Res<DirectionAtlas>,
        mut register:       ResMut<Registry>
    ) {
        /*
            Тут будет непосредственно инициализация мира, где будет размещение игровой сетки, основных его компонентов и сущностей.
            Установка синхронно с процессом загрузки ресурсов из файла.
        */

        // Test ==============================

        // Спавн спрайта, являющийся игроком
        if let Some(sprite) = register.get_entity("human", &atlas) {
            let entity = commands.spawn((
                RigidBody::Dynamic,
                EntityBase {
                    speed: Speed(50., 75., 25.),
                    health: Health(100.),
                    position: Position(Vec3::new(64., 64., 0.)),
                    direction: DirectionState::South,
                    movable: true,
                    ..default()
                },
                sprite,
                SpriteLayer::Entity,
                EntityType::Humonoid(HumonoidType::Human),
                EntityNeutrality::Neutral,
                Name::new("Player"),
            ))
            .insert(Velocity::zero())
            .insert(Collider::round_cuboid(2., 2., 0.25))
            .insert(LockedAxes::ROTATION_LOCKED)
            .id();
    
            commands.entity(entity)
            .insert(User {
                control_entity: Some(entity),
                ..default()
            })
            .insert(Container::default());

            // Спавн оружия и соединение с игроком
            if let Some(sprite) = register.get_test("gun", &atlas) {
                commands
                .spawn(SpriteSheetBundle {
                    texture: handle.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: TestTextureAtlas::get_index("gun", &handle),
                    },
                    transform: Transform {
                        translation: Vec3::new(0., 0., 0.5),
                        ..default()
                    },
                    ..default()
                })
                .insert(EntityObject::default())
                .insert(SpriteLayer::Item)
                .insert(PlayerAttach {
                    offset: Vec2::new(0., -3.),
                })
                .insert(GunController {
                    shoot_cooldown: 0.5,
                    shoot_timer: 0.,
                });
            } else {
                println!("ERROR - Spawn Gun!")
            }
        } else {
            println!("ERROR - Spawn Player!")
        }

        // Точка спавна для спавна "болванчиков", которые двигаются к игроку
        commands
            .spawn((
                SpriteSheetBundle {
                    texture: handle.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: TestTextureAtlas::get_index("test_square", &handle),
                    },
                    transform: Transform {
                        translation: Vec3::new(256.0, 256.0, 0.2),
                        ..default()
                    },
                    ..default()
                }
            ))
            .insert(EnemySpawner {
                is_active: false,
                cooldown: 5.,
                timer: 1.,
            });

    }

    /// Функция для инициализации загрузки чанков вокруг игрока в пределах установленной прогрузки.
    fn load_chunk_around(
        mut commands:       Commands,
            asset_server:   Res<AssetServer>,
        mut worldres:       ResMut<WorldRes>,
            handle:         Res<TestTextureAtlas>,
            player_query:   Query<(&mut Transform, &mut User)>,
        mut chunk_load:     EventWriter<LoadChunkPos>,
        mut chunk_upload:   EventWriter<DischargeChunkPos>
    ) {
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
            for (pos) in &worldres.chunk {
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
                chunk_upload.send(DischargeChunkPos(chunk));
                let chunk_list_len = worldres.chunk.len();
                for index in 0..chunk_list_len-1 {
                    if &chunk == worldres.chunk.get(index).unwrap() {
                        worldres.chunk.remove(index);
                    }
                }
            }

            for chunk in chunks_to_upload {
                chunk_load.send(LoadChunkPos(chunk));
                worldres.chunk.push(chunk);
            }

            // test
            for chunk in chunks_to_upload_test {
                Self::create_chunk(&mut commands, &asset_server, &mut worldres, &handle, chunk);
            }
            for chunk in chunks_to_discharge_test {
                Self::despawn_chunk(&mut commands, &mut worldres, chunk);
            }
        }
    }

    // ==============================
    // TEST
    // ==============================
    fn create_chunk(
        commands:       &mut Commands,
        asset_server:   &Res<AssetServer>,
        world_res:      &mut ResMut<WorldRes>,
        handle:         &Res<TestTextureAtlas>,
        pos:            IVec2
    ) -> Entity {
        let chunk = commands
            .spawn(SpriteSheetBundle {
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomLeft,
                    ..default()
                },
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: TestTextureAtlas::get_index("dirt", &handle),
                },
                transform: Transform {
                    translation: Vec3::new(pos.x as f32 * 256.0, pos.y as f32 * 256.0, -1.0),
                    scale: Vec3::new(16.0, 16.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .insert(Name::new(format!("{pos}_chunk")))
            .id();
        world_res.chunks.insert(pos, chunk);
        chunk
    }
    fn despawn_chunk(
        commands: &mut Commands,
        world: &mut ResMut<WorldRes>,
        //chunk: &Chunk,
        pos: IVec2,
    ) {
        if let Some(entity) = world.chunks.remove(&pos) {
            commands.entity(entity).despawn();
        } else {
            println!("despawn failed")
        }
    }

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
        let cell_size = 16;
        let result: IVec2 = IVec2::new(
            if input_var.x >= 0 {
                input_var.x / cell_size
            } else {
                (input_var.x - cell_size + 1) / cell_size
            },
            if input_var.y >= 0 {
                input_var.y / cell_size
            } else {
                (input_var.y - cell_size + 1) / cell_size
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
    pub fn get_local_tile_chunk(input_var: IVec2) {}
}

#[derive(Component, Resource)]
pub struct WorldRes {
    player_render_distance: i32,
    player_chunk_position: IVec2,
    player_chunk_last_position: IVec2,
    first_launch: bool,
    chunk_size_t: i32,

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
            chunk_size_t: 256,

            chunks: HashMap::new(),
            chunk: Vec::new()
        }
    }
}

impl WorldRes {
    pub fn new() -> Self {
        WorldRes { ..default() }
    }
}