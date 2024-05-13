use bevy::prelude::*;
use bevy_egui::{
    egui,
    EguiContexts
};

use crate::core::{
    interface::GameUI::GameUI, 
    ItemType::*, 
    Container::Container, 
    UserSystem::User
};

#[derive(Default, Resource)]
pub struct Console {
    line_input: String,
    input_buffer: Vec<String>,
    select_index: usize
}

pub fn toggle_console(
    mut parent_query:       Query<&mut GameUI, With<GameUI>>,
    mut contexts:           EguiContexts,
    mut console:            ResMut<Console>,
    mut event:              EventWriter<ConsoleInput>,
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

                            if ui.button(">").clicked() || 
                            (keyboard_input.just_pressed(KeyCode::NumpadEnter)) {
                                if !console.line_input.is_empty() {
                                    let text = console.line_input.clone();
                                    event.send(ConsoleInput(text.clone()));
                                    console.input_buffer.push(text);
                                    console.line_input.clear()
                                }
                            }

                            if keyboard_input.just_pressed(KeyCode::ArrowUp) {
                                if console.select_index < console.input_buffer.len() - 1 {
                                    console.select_index += 1;
                                }
                                console.line_input = console.input_buffer[console.select_index].clone()
                            }

                            if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                                if console.select_index > 0 {
                                    console.select_index -= 1;
                                }
                                console.line_input = console.input_buffer[console.select_index].clone()
                            }
                        });
                        
                    });
        } else {
            if !console.line_input.is_empty() {
                console.line_input.clear()
            }
        }
    }
}

#[derive(Event)]
pub struct ConsoleInput(pub String);

pub fn cmd_execute(
    mut _commands:  Commands,
    mut player:     Query<(&mut Transform, &mut Container), With<User>>,
    mut event:      EventReader<ConsoleInput>
) {
    if event.is_empty() {
        return;
    }

    for text in event.read() {
        if let Some('/') = text.0.chars().next() {
            let parts: Vec<&str> = text.0.split_whitespace().collect();
    
            if let Some(cmd) = parts.get(0) {
                match *cmd {
                    "/tp" => {
                        println!("/tp");
    
                        if let Some(x_str) = parts.get(1) {
                            if let Some(y_str) = parts.get(2) {
                                if let Ok(x) = x_str.parse::<f32>() {
                                    if let Ok(y) = y_str.parse::<f32>() {
                                        if let Ok(mut player) = player.get_single_mut() {
                                            player.0.translation = Vec3::new(x * 16.0, y * 16.0, 0.5);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "/size" => {
                        println!("/size");
    
                        if let Some(x_str) = parts.get(1) {
                            if let Ok(x) = x_str.parse::<f32>() {
                                if let Ok(mut player) = player.get_single_mut() {
                                    player.0.scale = Vec3::splat(x)
                                }
                            }
                        }
                    }
                    "/get" => {
                        println!("/get");

                        if let Some(x_str) = parts.get(1).map(|s| *s) {
                            if let Some(y_str) = parts.get(2) {
                                match x_str {
                                    "ammo" => {
                                        if let Ok(mut player) = player.get_single_mut() {
                                            if let Ok(y) = y_str.parse::<usize>() {
                                                player.1.add_in_container(ItemType::Item(Item::Ammo), y);
                                            }
                                        } 
                                    },
                                    _ => {
                                        println!("Неизвестная команда")
                                    }
                                }           
                            }
                        }
                    }
                    _ => {
                        println!("Неизвестная команда")
                    }
                }
            }
        }
    }
}