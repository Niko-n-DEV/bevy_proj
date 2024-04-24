//#![allow(unused)]
use bevy::{
    //input::mouse::{MouseMotion, MouseWheel},
    math::vec3,
    prelude::*,
};

use bevy_pancam::{
    PanCam,
    PanCamPlugin
};

use crate::core::{
    UserSystem::User,
    AppState
};

//use bevy_entitiles::tilemap::chunking::camera::CameraChunkUpdater;

// Основной компонент камеры
// Определить параметр зацепа к объекту (option)
#[derive(Component)]
pub struct CameraX;

#[derive(Resource)]
pub struct CameraXRes {
    pub player_fixed: bool,
}

impl Default for CameraXRes {
    fn default() -> Self {
        Self { player_fixed: true }
    }
}

// ===== Base =====
pub struct CameraController;

impl Plugin for CameraController {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PanCamPlugin::default())
            .add_systems(Startup, Self::setup_camera)
            .init_resource::<CameraXRes>()
            .add_systems(
                Update,
                Self::camera_follow_player.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                Self::toggle_camera_fix.run_if(in_state(AppState::Game)),
            );
    }
}

impl CameraController {
    fn setup_camera(mut commands: Commands) {
        commands
            .spawn((
                Camera2dBundle::default(),
                CameraX,
                //CameraChunkUpdater::new(1.3, 2.2),
            ))
            .insert(PanCam {
                grab_buttons: vec![],
                zoom_to_cursor: false,
                ..default()
            });
    }

    fn camera_follow_player(
        player_query: Query<&Transform, With<User>>,
        mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<User>)>,
        camera_res: Res<CameraXRes>,
    ) {
        if player_query.is_empty() || camera_query.is_empty() {
            return;
        }

        let mut camera_transform = camera_query.single_mut();
        let player_transform = player_query.single().translation;

        if camera_res.player_fixed {
            let (x, y) = (player_transform.x, player_transform.y);
            camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 0.1);
        }
    }

    fn toggle_camera_fix(
        mut camera_res: ResMut<CameraXRes>,
        mut cam: Query<&mut PanCam>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_released(KeyCode::F1) {
            let mut cam = cam.single_mut();
            if camera_res.player_fixed {
                cam.grab_buttons = vec![MouseButton::Middle];
                cam.zoom_to_cursor = true;

                camera_res.player_fixed = false
            } else {
                cam.grab_buttons = vec![];
                cam.zoom_to_cursor = false;
                
                camera_res.player_fixed = true
            }
        }
    }
}
