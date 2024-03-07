#![allow(unused)] // Удалить потом
use bevy::prelude::*;
use std::collections::HashMap;

use crate::core::world::TileMap::TileMapPlugin;

#[derive(Component)]
pub struct World {
    //chunks: HashMap<Vector2i, Gd<Node>>, // списко имеющиеся чанков координаты:(x,y), Chunk
    //chunk_manager: Option<ChunkManager>,
    player_render_distance: i32,
    player_chunk_position: IVec2,
    player_chunk_last_position: IVec2,
    first_launch: bool,
    chunk_size_t: i32,
}

pub struct WorldSystem;

impl Plugin for WorldSystem {
    fn build(&self, app: &mut App) {
        app.add_plugins(TileMapPlugin);
    }
}
