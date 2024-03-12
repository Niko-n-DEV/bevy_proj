#![allow(unused)] // Удалить потом
use bevy::prelude::*;

use crate::AppState;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
pub struct TileMap {
    grid_size: i32,
    tile_size: i32,
    
}

impl TileMap {
    
}