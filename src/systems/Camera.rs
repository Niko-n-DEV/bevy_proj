#![allow(unused)]
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    math::vec3,
    prelude::*,
};
use bevy_pancam::PanCam;

// test
// Для тестирования EntitiTiles
use crate::{entities::player::PlayerEntity::User, entities::Entity::EntityBase, AppState};

use bevy_entitiles::tilemap::chunking::camera::CameraChunkUpdater;

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

// test
// Для тестирования EntitiTiles
#[derive(Resource)]
pub struct CameraControl {
    pub target_pos: Vec2,
    pub target_scale: f32,
}

impl Default for CameraControl {
    fn default() -> Self {
        Self {
            target_pos: Default::default(),
            target_scale: 1.,
        }
    }
}

// test
// Для тестирования EntitiTiles
pub fn camera_control(
    mut query: Query<(&mut Transform, &mut OrthographicProjection)>,
    input_keyboard: Res<ButtonInput<KeyCode>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut event_wheel: EventReader<MouseWheel>,
    mut event_move: EventReader<MouseMotion>,
    time: Res<Time>,
    mut control: ResMut<CameraControl>,
) {
    let Ok((mut transform, mut projection)) = query.get_single_mut() else {
        return;
    };

    if input_mouse.pressed(MouseButton::Left) {
        for ev in event_move.read() {
            control.target_pos +=
                projection.scale * ev.delta * time.delta_seconds() * 200. * Vec2::new(-1., 1.);
        }
    } else {
        let mut step = 270. * time.delta_seconds();
        if input_keyboard.pressed(KeyCode::ShiftLeft) {
            step *= 2.;
        }

        let mut x = 0;
        if input_keyboard.pressed(KeyCode::KeyD) {
            x += 1;
        }
        if input_keyboard.pressed(KeyCode::KeyA) {
            x -= 1;
        }
        control.target_pos += Vec2::new(x as f32 * step, 0.);

        let mut y = 0;
        if input_keyboard.pressed(KeyCode::KeyW) {
            y += 1;
        }
        if input_keyboard.pressed(KeyCode::KeyS) {
            y -= 1;
        }
        control.target_pos += y as f32 * step * Vec2::Y;
    }

    let target = control.target_pos.extend(0.);
    if transform.translation.distance_squared(target) > 0.01 {
        transform.translation = transform
            .translation
            .lerp(target, 40. * time.delta_seconds());
    }

    for ev in event_wheel.read() {
        control.target_scale -= ev.y * 0.02;
        control.target_scale = control.target_scale.max(0.01);
    }

    if (projection.scale - control.target_scale).abs() > 0.01 {
        projection.scale = projection.scale
            + ((control.target_scale - projection.scale) * 20. * time.delta_seconds());
    }
    event_move.clear();
}

// ===== Base =====
pub struct CameraController;

impl Plugin for CameraController {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup_camera)
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
                grab_buttons: vec![MouseButton::Middle],
                ..default()
            })
            .id();
    }

    fn camera_follow_player(
        mut player_query: Query<&Transform, With<User>>,
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
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_released(KeyCode::F1) {
            if camera_res.player_fixed {
                camera_res.player_fixed = false
            } else {
                camera_res.player_fixed = true
            }
        }
    }
}
