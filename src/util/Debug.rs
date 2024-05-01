#![allow(unused)]
use std::fmt;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::{Component, Query, Res, With},
    text::Text,
};

#[allow(dead_code)]
#[derive(Component)]
pub struct DebugFpsText;

#[allow(dead_code)]
pub fn debug_info_display(
    mut query: Query<&mut Text, With<DebugFpsText>>,
    diag: Res<DiagnosticsStore>,
) {
    if let (Some(fps), Some(frame_time)) = (
        diag.get(&FrameTimeDiagnosticsPlugin::FPS),
        diag.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME),
    ) {
        if let (Some(fps_value), Some(frame_time_value)) = (fps.smoothed(), frame_time.smoothed()) {
            let mut text = query.get_single_mut().unwrap();
            text.sections[1].value = format!("{fps_value:.2} ({frame_time_value:.2} ms)");
        }
    }
}

// ==============================
// Debug - Error
// ==============================

#[derive(Debug)]
pub struct GameError {
    pub error_type: GameErrorType,
    pub error_payload: String,
}

impl GameError {
    pub fn new(error_type: GameErrorType, payload: String) -> Self {
        GameError {
            error_type,
            error_payload: payload,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum GameErrorType {
    ItemMissing,
    CraftingFailed,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.error_payload)
    }
}
