#![allow(unused)] // Удалить потом
use bevy::prelude::*;
use bevy_entitiles::*;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {}
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}
