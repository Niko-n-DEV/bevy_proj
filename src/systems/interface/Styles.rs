use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.23, 0.23, 0.23);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.26, 0.26, 0.26);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.29, 0.29, 0.29);

// pub const MAIN_MENU_STYLE: Style = Style {
//     flex_direction: FlexDirection::Column,
//     justify_content: JustifyContent::Center,
//     align_items: AlignItems::Center,
//     height: Val::Percent(100.0),
//     width: Val::Percent(100.0),
//     ..Style::DEFAULT
// };

// pub const BUTTON_STYLE: Style = Style {
//     justify_content: JustifyContent::Center,
//     align_items: AlignItems::Center,
//     size: Size::new(Val::Px(200.0), Val::Px(80.0)),
//     ..Style::DEFAULT
// };

// pub const IMAGE_STYLE: Style = Style {
//     size: Size::new(Val::Px(64.0), Val::Px(64.0)),
//     margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
//     ..Style::DEFAULT
// };

// pub const TITLE_STYLE: Style = Style {
//     flex_direction: FlexDirection::Row,
//     justify_content: JustifyContent::Center,
//     align_items: AlignItems::Center,
//     size: Size::new(Val::Px(300.0), Val::Px(120.0)),
//     ..Style::DEFAULT
// };


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