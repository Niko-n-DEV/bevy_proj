#![allow(non_snake_case)]
pub mod TileMap;
pub mod World;

pub mod chunk;

use bevy::prelude::*;

pub struct WorldTaskManager;

impl Plugin for WorldTaskManager {
    fn build(&self, _app: &mut App) {}
}
