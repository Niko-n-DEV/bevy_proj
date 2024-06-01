#![allow(unused)]
use bevy::{
    //input::mouse::{MouseMotion, MouseWheel},
    math::vec3,
    prelude::*,
};

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use bevy_pancam::{
    PanCam,
    PanCamPlugin
};

use bevy_pixel_camera::{
    PixelCameraPlugin,
    PixelViewport
};

use crate::core::{
    UserSystem::{
        UserControl,
        User,
    },
    AppState
};

//use bevy_entitiles::tilemap::chunking::camera::CameraChunkUpdater;

// Основной компонент камеры
// Определить параметр зацепа к объекту (option)
#[derive(Resource)]
pub struct UserCameraRes {
    pub player_fixed: bool,
    pub coef: f32
}

#[derive(Component)]
pub struct UserCamera;

// ===== Base =====
pub struct CameraController;

impl Plugin for CameraController {
    fn build(&self, app: &mut App) {
        app
            // Register Types
            // Init Resource
            .insert_resource(UserCameraRes { player_fixed: false, coef: 0.066 })
            // Init Plugins
            .add_plugins(PanCamPlugin::default())
            .add_plugins(PixelCameraPlugin)
            // Init Systems
            .add_systems(Startup, Self::setup_camera)
            .add_systems(OnEnter(AppState::Game), Self::toggle_camera_options)
            .add_systems(PostUpdate, Self::camera_follow_player.run_if(in_state(AppState::Game)))
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
                PixelViewport,
                UserCamera
            ))
            .insert(PanCam {
                enabled: false,
                grab_buttons: vec![],
                zoom_to_cursor: false,
                min_scale: 0.05,
                max_scale: Some(1.0),
                ..default()
            });
    }

    /// Переключает возможности камеры PanCam (Приближение и т.д.) при переходе между сценами.
    fn toggle_camera_options(
        mut cam:    Query<(&mut PanCam)>,
        mut u_cam:  ResMut<UserCameraRes>,
            user:   Res<User>
    ) {
        if let Ok(mut cam) = cam.get_single_mut() {
            cam.enabled = !cam.enabled;
            if user.control_entity.is_none() {
                u_cam.player_fixed = false
            } else {
                u_cam.player_fixed = true
            }
        }
    }

    fn camera_follow_player(
        mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<UserControl>)>,
            player_query: Query<&Transform, With<UserControl>>,
            user_camera:  Res<UserCameraRes>
    ) {
        if player_query.is_empty() || camera_query.is_empty() {
            return;
        }

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            if user_camera.player_fixed {
                let player_transform = player_query.single().translation;
                let (x, y) = (player_transform.x, player_transform.y);
                camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), user_camera.coef);
            }
        }
    }

    fn toggle_camera_fix(
        mut cam_query:      Query<(&mut PanCam)>,
            keyboard_input: Res<ButtonInput<KeyCode>>,
        mut user_camera:    ResMut<UserCameraRes>,
            user:           Res<User>
    ) {
        if keyboard_input.just_released(KeyCode::F1) {
            if let Ok(mut cam) = cam_query.get_single_mut() {
                if user_camera.player_fixed {
                    cam.grab_buttons = vec![MouseButton::Middle];
                    cam.zoom_to_cursor = true;
                    cam.min_scale = 0.1;
                    cam.max_scale = Some(0.5);
                } else {
                    cam.grab_buttons = vec![];
                    cam.zoom_to_cursor = false;
                    cam.min_scale = 0.05;
                    cam.max_scale = Some(1.0);
                }
                user_camera.player_fixed = !user_camera.player_fixed;
            }
        }

        if user.control_entity.is_none() && !user_camera.player_fixed {
            if let Ok(mut cam) = cam_query.get_single_mut() {
                cam.grab_buttons = vec![MouseButton::Middle];
                cam.zoom_to_cursor = true;
            }
        }
    }
}
