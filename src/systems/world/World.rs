#![allow(unused)] // Удалить потом
use bevy::{log::tracing_subscriber::fmt::format, prelude::*, transform::commands};
use std::collections::HashMap;

use crate::core::{
    AppState,
    player::PlayerEntity::PlayerEntity,
    graphic::Atlas::{TestTextureAtlas, DirectionAtlas},
    Settings::Settings
};

pub struct WorldSystem;

impl Plugin for WorldSystem {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), Self::setup)
            .init_resource::<WorldRes>()
            .add_systems(Update, Self::load_chunk_around.run_if(in_state(AppState::Game)));
    }
}

impl WorldSystem {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut world: ResMut<WorldRes>) {
        let settings = Settings::load();
        world.player_render_distance = settings.rendering_distance;
        settings.save();
    }

    fn init_world(
        mut commands: Commands,
        handle: Res<TestTextureAtlas>,
        handle_dir: Res<DirectionAtlas>
    ) {
        /*
            Тут будет непосредственно инициализация мира, где будет размещение игровой сетки, основных его компонентов и сущностей.
            Установка синхронно с процессом загрузки ресурсов из файла.
        */
    }

    /// Функция для инициализации загрузки чанков вокруг игрока в пределах установленной прогрузки.
    fn load_chunk_around(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut worldres: ResMut<WorldRes>,
        handle: Res<TestTextureAtlas>,
        player_query: Query<(&mut Transform, &mut PlayerEntity)>,
    ) {
        /*
                По сути потоковая функция, которая будет прогружать территорию
                Небольшой черновик работы функции.
                Отправляется от сущности "Игрок", вычисляется где он находиться и прогружает во круг чанки, если они находяться в зоне прогрузки.

                Изберательная прогрузка: У игрока есть некая зона, в пределах которой не прогружается далее территория, если зайти за предел радиуса допуска, то происходит загрузка.
                Та некая зона появляется, если стоять на месте некоторое время (зона афк))
        d
                При процессе загрузки чанков, сначала происходит проверка [check_exists_chunk], есть ли чанк, если нету происходит генерация [generate_chunk], если же чанк был найден, происходит загрузка чанка [load_chunk], создавая экземпляр
                сцены в res//, с именем, который является его номером по координатной сетке (0_0 | 0_1 | 1_0 и т.д.). Сам чанк, сохраняется и выгружается из файла, который представляет собой своеобразный сжатый "архив",
                хронящий информацию о 32x32 чанках по координатной сетке, т.е. есть "архив" в нём 32x32 чанка, он занимает положение в мире 0_0 и т.д.
                */

        for (transform, _player) in &player_query {
            let player_translation = transform.translation.truncate().as_ivec2();
            worldres.player_chunk_position = Self::get_current_chunk(player_translation)
        }

        if worldres.player_chunk_position == worldres.player_chunk_last_position && worldres.first_launch {
            worldres.player_chunk_last_position = IVec2::ZERO;
            worldres.first_launch = false
        }

        if worldres.player_chunk_position != worldres.player_chunk_last_position {
            worldres.player_chunk_last_position = worldres.player_chunk_position;

            let (player_chunk_x, player_chunk_y) =
                (worldres.player_chunk_position.x, worldres.player_chunk_position.y);

            // Нужна более чательная проработка
            let mut loaded_chunks_new: Vec<IVec2> = Vec::new();
            for x in (player_chunk_x - worldres.player_render_distance)
                ..=(player_chunk_x + worldres.player_render_distance)
            {
                for y in (player_chunk_y - worldres.player_render_distance)
                    ..=(player_chunk_y + worldres.player_render_distance)
                {
                    //let distance = ((player_chunk_x - x).abs() + (player_chunk_y - y).abs()).max(1);
                    // if distance <= world.player_render_distance {
                    //     let chunk_pos = IVec2::new(x, y);
                    //     loaded_chunks_new.push(chunk_pos);
                    // }
                    let chunk_pos = IVec2::new(x, y);
                    loaded_chunks_new.push(chunk_pos);
                }
            }

            let mut chunks_to_discharge: Vec<IVec2> = Vec::new();
            // Проверяет в chunk, есть ли чанки, которые не входят в радиус прогрузки, чтобы их выгрузить
            for (pos, _) in &worldres.chunks {
                if !loaded_chunks_new.contains(pos) {
                    chunks_to_discharge.push(*pos);
                }
            }
            // Проверяет, есть ли чанки в списке на прогрузку, которые ещё не загружены, чтобы их загрузить.
            let mut chunks_to_upload: Vec<IVec2> = Vec::new();
            for chunk in &loaded_chunks_new {
                if !worldres.chunks.contains_key(chunk) {
                    chunks_to_upload.push(*chunk);
                }
            }

            for chunk in chunks_to_upload {
                Self::create_chunk(&mut commands, &asset_server, &mut worldres, &handle, chunk);
            }

            for chunk in chunks_to_discharge {
                Self::despawn_chunk(&mut commands, &mut worldres, chunk);
            }
        }
    }

    //временно
    fn create_chunk(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        world_res: &mut ResMut<WorldRes>,
        handle: &Res<TestTextureAtlas>,
        pos: IVec2,
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
                    index: TestTextureAtlas::get_index("dirt", &handle)
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
        //let result: IVec2 = IVec2::new(input_var.x / 256, input_var.y / 256);
        let result = Self::get_format_current_chunk(input_var);
        //println!("{} | {}", result, input_var);
        result
    }

    /// Функция для форматирования значения чанков по координатной системе
    fn get_format_current_chunk(input_var: IVec2) -> IVec2 {
        //IVec2::new(input_var.x / 256 + if input_var.x % 256 < 0 { -1 } else { 0 }, input_var.y / 256 + if input_var.y % 256 < 0 { -1 } else { 0 }) // Godot version, incurrent
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
    pub fn get_currect_chunk_tile(input_var: IVec2) {}

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
        }
    }
}

impl WorldRes {
    pub fn new() -> Self {
        WorldRes { ..default() }
    }
}

#[derive(Component, Resource)]
struct Chunk {
    pub chunk_pos: IVec2,
    pub entity: Entity,
}

impl Chunk {
    pub fn new(chunk_pos: IVec2, entity: Entity) -> Self {
        Self { chunk_pos, entity }
    }
}
