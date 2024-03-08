#![allow(unused)] // Удалить потом
use bevy::prelude::*;

use crate::{entities::player::PlayerEntity::PlayerEntity, AppState};

#[derive(Component)]
pub struct CameraX;
pub struct CameraController;

impl Plugin for CameraController {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, Self::setup_camera)
        //    .add_systems(Update, Self::camera_follow_player.run_if(in_state(AppState::Game)))
        ;
    }
}

impl CameraController {
    fn setup_camera(mut commands: Commands) {
        commands.spawn((Camera2dBundle::default(), CameraX)).id();
    }

    fn camera_follow(
        mut follower_query: Query<(&mut Transform, &CameraX)>,
        camera_query: Query<&Transform, (With<Camera2d>, Without<CameraX>)>,
    ) {
        let camera_translation = camera_query.single().translation;
        for (mut transform, _) in follower_query.iter_mut() {
            transform.translation.x = camera_translation.x;
            transform.translation.y = camera_translation.y;
        }
    }

    fn camera_follow_player(
        player_query: Query<&Transform, With<PlayerEntity>>,
        mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<PlayerEntity>)>,
    ) {
        let player_tranform = player_query.single().translation;
        let mut camera_transform = camera_query.single_mut();

        camera_transform.translation.x = player_tranform.x;
        camera_transform.translation.y = player_tranform.y;
    }
}
