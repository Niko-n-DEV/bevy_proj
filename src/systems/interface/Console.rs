use bevy::prelude::*;
use bevy_egui::{
    egui,
    EguiContexts
};

use crate::core::interface::GameUI::GameUI;

#[derive(Default, Resource)]
pub struct Console {
    line_input: String,
    //input_buffer: Vec<String>
}

pub fn toggle_console(
    mut parent_query:       Query<&mut GameUI, With<GameUI>>,
    mut contexts:           EguiContexts,
    mut console:            ResMut<Console>,
        keyboard_input:     Res<ButtonInput<KeyCode>>,
) {
    if parent_query.is_empty() {
        return;
    }

    if let Ok(mut game_ui) = parent_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Enter) {
            game_ui.console_toggle = !game_ui.console_toggle;
        }

        if game_ui.console_toggle {
            egui::Window::new("Console")
                    .show(contexts.ctx_mut(), |ui| {
                        
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut console.line_input);
                            if ui.button(">").clicked() {
                                console.line_input.clear()
                            }
                        });
                        
                    });
        }
    }
    
}