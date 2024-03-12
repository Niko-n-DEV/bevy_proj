#![allow(unused)] // Удалить потом
use bevy::{prelude::*, transform::commands};
use std::collections::HashMap;

use crate::{core::{player::PlayerEntity::PlayerEntity, world::TileMap::TileMapPlugin}, AppState};

pub struct WorldSystem;

impl Plugin for WorldSystem {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), Self::setup)
            .add_systems(Update, Self::load.run_if(in_state(AppState::Game)));
    }
}

impl WorldSystem {
    fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        let mut world = World::new();
        commands.insert_resource(world);
    }

    /// Функция для инициализации загрузки чанков вокруг игрока в пределах установленной прогрузки.
    fn load(
        mut commands: Commands,
        player_pos: Query<(&mut Transform, &mut PlayerEntity)>
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

        for (transform, player) in &player_pos {
            let player_pos = transform.translation.truncate().as_ivec2();
            Self::get_current_chunk(player_pos)
        }

        //let player_pos = player_pos.single().translation.truncate().as_ivec2();

        
    }

    /// Функция для определения точных координат чанка
    /// 
    /// Определяется по данной позиции и делением на общий размер чанка
    pub fn get_current_chunk(
        input_var: IVec2
    ) {
        let result: IVec2 = IVec2::new(input_var.x / 256, input_var.y / 256);
        let current_result = Self::get_format_current_chunk(input_var);
        //place_image_chunk(commands, asset_server, current_result.x as f32, current_result.y as f32);
        println!("{} | {} | {}", result, current_result, input_var)
    }

    /// Функция для форматирования значения чанков по координатной системе
    fn get_format_current_chunk(input_var: IVec2) -> IVec2 {
        IVec2::new(input_var.x / 256 + if input_var.x % 256 < 0 { -1 } else { 0 }, input_var.y / 256 + if input_var.y % 256 < 0 { -1 } else { 0 })
    }

    /// Функция для определения точных координат тайла в чанке
    /// 
    /// Определяется по данной позиции и позиции чанка, делением на общий размер одного тайла
    pub fn get_currect_chunk_tile(input_var: IVec2) {

    }

    /// Функция для определения координат тайла в пределах чанка
    /// 
    /// Определяется по данной позиции и определением координат в пределах одного чанка, где отсчёт координат начинается с верхнего левого угла чанка.
    pub fn get_local_tile_chunk(input_var: IVec2) {

    }
}


#[derive(Component, Resource)]
pub struct World {
    player_render_distance: i32,
    player_chunk_position: IVec2,
    player_chunk_last_position: IVec2,
    first_launch: bool,
    chunk_size_t: i32,

    chunks_list: Vec<Chunk>
}

struct Chunk {
    x: i32,
    y: i32
}

impl Default for World {
    fn default() -> Self {
        Self {
            player_render_distance: 3,
            player_chunk_position: IVec2 { x: 0, y: 0 },
            player_chunk_last_position: IVec2 { x: 0, y: 0 },
            first_launch: true,
            chunk_size_t: 256,

            chunks_list: Vec::new()
        }
    }
}

impl World {
    pub fn new() -> Self {
        World {
            chunks_list: Vec::new(),
            ..default()
        }
    }

    /// Функция для инициализации загрузки чанков вокруг игрока в пределах установленной прогрузки.
    pub fn load(
        commands: &mut Commands,
        player_pos: Query<&Transform, With<PlayerEntity>>
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

        let player_pos = player_pos.single().translation.truncate().as_ivec2();

        Self::get_current_chunk(player_pos)
    }

    /// Функция для определения точных координат чанка
    /// 
    /// Определяется по данной позиции и делением на общий размер чанка
    pub fn get_current_chunk(
        input_var: IVec2
    ) {
        let result: IVec2 = IVec2::new(input_var.x / 256, input_var.y / 256);
        let current_result = Self::get_format_current_chunk(input_var);
        //place_image_chunk(commands, asset_server, current_result.x as f32, current_result.y as f32);
        println!("{} | {} | {}", result, current_result, input_var)
    }

    fn get_format_current_chunk(input_var: IVec2) -> IVec2 {
        IVec2::new(input_var.x / 256 + if input_var.x % 256 < 0 { -1 } else { 0 }, input_var.y / 256 + if input_var.y % 256 < 0 { -1 } else { 0 })
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

// fn place_image_chunk(
//     commands: &mut Commands,
//     asset_server: &Res<AssetServer>,
//     x: f32,
//     y: f32
// ) {
//     commands.spawn(
//         SpriteBundle {
//             sprite: Sprite {
//                 anchor: bevy::sprite::Anchor::TopLeft,
//                 ..default()
//             },
//             texture: asset_server.load("dirt.png"),
//             transform: Transform {
//                 translation: Vec3::new(x * 256.0, y * 256.0, 0.0),
//                 scale: Vec3::new(16.0, 16.0, 0.0),
//                 ..default()
//             },
//             ..default()
//         });
// }