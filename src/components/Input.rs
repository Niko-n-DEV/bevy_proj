use bevy::{prelude::*, window::PrimaryWindow};

use crate::systems::Camera::CameraX;

#[derive(Resource)]
pub struct OffsetedCursorPosition {
    pub x: f32,
    pub y: f32,
}

/// Позиция курсора на глобальной координатной сетке
#[derive(Resource, Default)]
pub struct CursorPosition(pub Vec2);

/// Получение координат чанка по глобальной координатной системе
pub fn cursor_track(
    mut cursor_pos: ResMut<CursorPosition>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<CameraX>>
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_pos.0 = world_position;
        //eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}