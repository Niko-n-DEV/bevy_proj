#![allow(unused)]
use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.23, 0.23, 0.23);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.26, 0.26, 0.26);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.29, 0.29, 0.29);

pub const DARK_GRAY_COLOR: Color = Color::rgb(0.12, 0.12, 0.12);
pub const DARK_LGRAY_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const DARK_LLGRAY_COLOR: Color = Color::rgb(0.20, 0.20, 0.20);

pub const GREEN_BUTTON_COLOR: Color = Color::rgb(0.31, 0.79, 0.47);

pub fn button_container_style(height: f32, width: f32) -> Style {
    Style {
        height: Val::Px(height),
        width: Val::Px(width),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    }
}
