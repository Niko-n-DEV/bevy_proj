#![allow(unused)]
use bevy::prelude::*;

use bevy_egui::{
    egui::{
        self, 
        style::HandleShape, 
        Color32, 
        Rounding, 
        Stroke
    }, 
    EguiContexts
};

pub const NORMAL_BUTTON_COLOR: Color    = Color::rgb(0.28, 0.29, 0.29);
pub const HOVERED_BUTTON_COLOR: Color   = Color::rgb(0.26, 0.26, 0.26);
pub const PRESSED_BUTTON_COLOR: Color   = Color::rgb(0.22, 0.23, 0.23);
pub const BORDER_BUTTON_COLOR: Color    = Color::rgb(0.36, 0.37, 0.38);
pub const BORDER_HOVER_COLOR: Color     = Color::rgb(0.99, 0.99, 0.99);

pub const DARK_GRAY_COLOR: Color        = Color::rgb(0.12, 0.12, 0.12);
pub const DARK_GRAY_BACK_COLOR: Color   = Color::rgb(0.10, 0.10, 0.10);
pub const DARK_LGRAY_COLOR: Color       = Color::rgb(0.15, 0.15, 0.15);
pub const DARK_LLGRAY_COLOR: Color      = Color::rgb(0.20, 0.20, 0.20);

pub const GREEN_BUTTON_COLOR: Color     = Color::rgb(0.31, 0.79, 0.47);

pub const BASE_UI_COLOR: Color          = Color::rgb(0.16, 0.16, 0.16);
pub const BASE_BORDER_UI_COLOR: Color   = Color::rgb(0.19, 0.19, 0.19);
pub const BASE_EX_UI_COLOR: Color       = Color::rgb(0.23, 0.23, 0.23);
pub const WIDGET_UI_COLOR: Color        = Color::rgb(0.20, 0.20, 0.20);

pub const BTN_COLOR: Color              = Color::rgb(0.56, 0.56, 0.56);
pub const BTN_BORDER_COLOR: Color       = Color::rgb(0.63, 0.63, 0.63);
pub const BTN_HOVER_COLOR: Color        = Color::rgb(0.69, 0.69, 0.69);
pub const BTN_PRESS_COLOR: Color        = Color::rgb(0.52, 0.52, 0.52);

pub const HEALTH_COLOR: Color           = Color::rgb(0.79, 0.15, 0.15);

pub const BASE_LINE_UI_COLOR: Color     = Color::rgb(0.23, 0.23, 0.23);

pub fn button_container_style(height: f32, width: f32) -> Style {
    Style {
        height:             Val::Px(height),
        width:              Val::Px(width),
        justify_content:    JustifyContent::Center,
        align_items:        AlignItems::Center,
        border:             UiRect::all(Val::Px(2.0)),
        margin:             UiRect::all(Val::Px(5.0)),
        ..default()
    }
}

/// Style for egui
pub fn setup_egui_style(mut ctx: EguiContexts) {
    ctx.ctx_mut().style_mut(|style| {
        let visuals = &mut style.visuals;
        let round = Rounding::from(2.);

        visuals.window_rounding = round;
        visuals.widgets.noninteractive.rounding = round;
        visuals.widgets.inactive.rounding = round;
        visuals.widgets.hovered.rounding = round;
        visuals.widgets.active.rounding = round;
        visuals.widgets.open.rounding = round;
        visuals.window_rounding = round;
        visuals.menu_rounding = round;

        visuals.collapsing_header_frame = true;
        visuals.handle_shape = HandleShape::Rect { aspect_ratio: 0.5 };
        visuals.slider_trailing_fill = true;

        visuals.widgets.hovered.bg_stroke = Stroke::new(2.0, Color32::from_white_alpha(180));
        visuals.widgets.active.bg_stroke = Stroke::new(3.0, Color32::WHITE);

        visuals.widgets.inactive.weak_bg_fill = Color32::from_white_alpha(10); // button
        visuals.widgets.hovered.weak_bg_fill = Color32::from_white_alpha(20); // button hovered
        visuals.widgets.active.weak_bg_fill = Color32::from_white_alpha(60); // button pressed

        visuals.selection.bg_fill = Color32::from_rgb(27, 76, 201);
        visuals.selection.stroke = Stroke::new(2.0, color32_gray_alpha(1., 0.78)); // visuals.selection.bg_fill

        visuals.extreme_bg_color = color32_gray_alpha(0.02, 0.66); // TextEdit, ProgressBar, ScrollBar Bg, Plot Bg

        visuals.window_fill = color32_gray_alpha(0.1, 0.99);
        visuals.window_shadow = egui::epaint::Shadow {
            blur: 8.,
            color: Color32::from_black_alpha(45),
            ..default()
        };
        visuals.popup_shadow = visuals.window_shadow;
    });

    // let mut fonts = FontDefinitions::default();
    // fonts.font_data.insert(
    //     "my_font".to_owned(),
    //     FontData::from_static(include_bytes!("../../../assets/fonts/menlo.ttf")),
    // );

    // // Put my font first (highest priority):
    // fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());

    // // Put my font as last fallback for monospace:
    // fonts.families.get_mut(&FontFamily::Monospace).unwrap().push("my_font".to_owned());

    // ctx.ctx_mut().set_fonts(fonts);
}

pub fn color32_gray_alpha(gray: f32, alpha: f32) -> Color32 {
    let g = (gray * 255.) as u8;
    let a = (alpha * 255.) as u8;
    Color32::from_rgba_premultiplied(g, g, g, a)
}