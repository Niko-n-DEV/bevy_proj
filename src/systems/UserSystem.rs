#![allow(unused)]
use bevy::{
    prelude::*, 
    render::camera::RenderTarget, 
    window::PrimaryWindow
};

#[derive(Component)]
pub struct Selectable {
    pub is_selected: bool
}

#[derive(Resource)]
pub struct SelectBox {
    pub is_active: bool,
    pub start_position: Vec2,
    pub current_position: Vec2
}

pub fn update_select_box(
    mut query: Query<(
        &Camera2d,
        &Camera,
        &GlobalTransform
    )>,
    mut select_box: ResMut<SelectBox>,
    mouse_input:    Res<ButtonInput<MouseButton>>,
    windows:        Query<&Window, With<PrimaryWindow>>
) {
    let (_, camera, global_transform) = query.single_mut();

    // let window = if let RenderTarget::Window(id) = camera.target {
    //     windows.get(id).unwrap()
    // } else {
    //     windows.get_primary().unwrap()
    // };

    let window = windows.single();

    if let Some(win_pos)    = window.cursor_position() {
        let window_size     = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc             = (win_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world    = global_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos       = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2         = world_pos.truncate();

        if mouse_input.just_pressed(MouseButton::Left) {
            select_box.is_active = true;
            select_box.start_position = world_pos;
            select_box.current_position = world_pos;
        }

        if mouse_input.pressed(MouseButton::Left) {
            select_box.is_active = true;
            select_box.current_position = world_pos;
        }

        if mouse_input.just_released(MouseButton::Left) {
            select_box.is_active = false;
        }
    }
}