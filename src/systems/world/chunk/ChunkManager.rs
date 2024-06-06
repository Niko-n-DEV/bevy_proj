#![allow(unused)]
use bevy::prelude::*;

pub fn chunk_manager_plugin(app: &mut App) {

}

#[derive(Event)]
pub struct ChunksToUpload;

#[derive(Event)]
pub struct ChunksToDischarge; 

fn check_exists_chunk() {}

fn load_chunk() {}

fn discharge_chunk() {}