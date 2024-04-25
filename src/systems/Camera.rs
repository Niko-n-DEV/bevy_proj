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
pub struct UserCamera {
    pub player_fixed: bool
}

// ===== Base =====
pub struct CameraController;

impl Plugin for CameraController {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PanCamPlugin::default())
            .add_systems(Startup, Self::setup_camera)
            .add_systems(OnEnter(AppState::Game), Self::toggle_camera_options)
            .add_systems(Update, Self::camera_follow_player.run_if(in_state(AppState::Game)))
            .add_systems(Update, Self::toggle_camera_fix.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), Self::toggle_camera_options)
        ;
    }
}

impl CameraController {
    fn setup_camera(mut commands: Commands) {
        commands
            .spawn((
                Camera2dBundle::default(),
                UserCamera {
                    player_fixed: false
                }
            ))
            .insert(PanCam {
                enabled: false,
                grab_buttons: vec![],
                zoom_to_cursor: false,
                ..default()
            });
    }

    /// Переключает возможности камеры PanCam (Приближение и т.д.) при переходе между сценами.
    fn toggle_camera_options(
        mut cam: Query<&mut PanCam>
    ) {
        if let Ok(mut cam) = cam.get_single_mut() {
            cam.enabled = !cam.enabled;
        }
    }

    fn camera_follow_player(
        player_query: Query<&Transform, With<User>>,
        mut camera_query: Query<(&mut Transform, &UserCamera), (With<Camera2d>, Without<User>)>
    ) {
        if player_query.is_empty() || camera_query.is_empty() {
            return;
        }

        let mut camera_transform = camera_query.single_mut();
        let player_transform = player_query.single().translation;

        if camera_transform.1.player_fixed {
            let (x, y) = (player_transform.x, player_transform.y);
            camera_transform.0.translation = camera_transform.0.translation.lerp(vec3(x, y, 0.0), 0.1);
        }
    }

    fn toggle_camera_fix(
        mut cam: Query<(&mut PanCam, &mut UserCamera)>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_released(KeyCode::F1) {
            let mut cam = cam.single_mut();
            if cam.1.player_fixed {
                cam.0.grab_buttons = vec![MouseButton::Middle];
                cam.0.zoom_to_cursor = true;

                cam.1.player_fixed = false
            } else {
                cam.0.grab_buttons = vec![];
                cam.0.zoom_to_cursor = false;
                
                cam.1.player_fixed = true
            }
        }
    }
}
