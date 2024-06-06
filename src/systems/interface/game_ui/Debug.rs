use bevy::prelude::*;

use bevy_egui::{
    egui, 
    EguiContexts
};

use crate::core::{
    Entity::EntityBase,
    entities::EntitySystem::EnemySpawner, 
    resource::Registry::Registry, 
    world::World::WorldSystem, 
    UserSystem::{
        CursorMode, 
        CursorPlacer, 
        UserControl
    },
    AppState
};

use super::GameUI;

pub fn debug_ui_plugin(app: &mut App) {
    app.add_systems(Update, DebugInfoPanel::toggle_debug_window.run_if(in_state(AppState::Game)));
}

// ========== DEBUG ==========
pub struct DebugInfoPanel;

impl DebugInfoPanel {
    // ==========
    // Функционал для открытия дебага
    // ==========
    fn toggle_debug_window(
        mut parent_query:   Query<&mut GameUI, With<GameUI>>,
        mut contexts:       EguiContexts,
        mut spawners:       Query<&mut EnemySpawner, With<EnemySpawner>>,
        mut placer:         ResMut<CursorPlacer>,
        mut cursor_mode:    ResMut<CursorMode>,
            registry:       Res<Registry>,
            player:         Query<&EntityBase, With<UserControl>>,
            keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if parent_query.is_empty() {
            return;
        }

        if let Ok(mut game_ui) = parent_query.get_single_mut() {
            if keyboard_input.just_pressed(KeyCode::F3) {
                game_ui.debug_menu = false;
                game_ui.debug_toggle = !game_ui.debug_toggle
            }

            if game_ui.debug_toggle {
                egui::Window::new("Debug")
                    .show(contexts.ctx_mut(), |ui| {
                        ui.label("Position");
                        if let Ok(player_pos) = player.get_single() {
                            ui.vertical(|ui| {
                                ui.label(format!("Pos: {}", player_pos.position.0));
                                ui.label(format!("Pos_T: {}", WorldSystem::get_currect_chunk_tile(player_pos.position.0.as_ivec2())));
                                ui.label(format!("Pos_C: {}", WorldSystem::get_current_chunk(player_pos.position.0.as_ivec2())));
                            });
                        }

                        ui.horizontal(|ui| {
                            if ui.button("Spawners").clicked() && !spawners.is_empty() {
                                for mut spawner in spawners.iter_mut() {
                                    spawner.is_active = !spawner.is_active
                                }
                            }
                        });

                        if ui.button("menu").clicked() {
                            game_ui.debug_menu = !game_ui.debug_menu;
                        }
                    });
            }

            if game_ui.debug_menu {
                egui::Window::new("Debug Menu")
                    .show(contexts.ctx_mut(), |ui| {
                        ui.label("Items");
                        ui.horizontal(|ui| {
                            for key in registry.item_registry.keys() {
                                if ui.button(key).clicked() {
                                    *cursor_mode = CursorMode::Placer;
                                    placer.placer = Some(("item".to_string(), key.clone()));
                                }
                            }
                        });

                        ui.label("Objects");
                        ui.horizontal(|ui| {
                            for key in registry.object_registry.keys() {
                                if ui.button(key).clicked() {
                                    *cursor_mode = CursorMode::Placer;
                                    placer.placer = Some(("object".to_string(), key.clone()));
                                }
                            }
                        });
                        ui.horizontal(|ui| {
                            for key in registry.object_ct_registry.keys() {
                                if ui.button(key).clicked() {
                                    *cursor_mode = CursorMode::Placer;
                                    placer.placer = Some(("object".to_string(), key.clone()));
                                }
                            }
                        });

                        ui.label("Entities");
                        ui.horizontal(|ui| {
                            for key in registry.entity_registry.keys() {
                                if ui.button(key).clicked() {
                                    *cursor_mode = CursorMode::Placer;
                                    placer.placer = Some(("entity".to_string(), key.clone()));
                                }
                            }
                        });
                    });
                
            }
        }
    }
}