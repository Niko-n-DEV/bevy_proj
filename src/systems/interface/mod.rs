#![allow(non_snake_case)]
pub mod Console;
pub mod GameUI;
pub mod Inventory;
//pub mod LogoUi;
pub mod MenuUI;
pub mod Styles;
pub mod UI;

use bevy::prelude::*;
use bevy_simple_text_input::TextInputPlugin;

use crate::core::AppState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Init resources
            .init_resource::<Console::Console>()
            // Register Types (Inspector)
            .register_type::<Inventory::InventoryGui>()
            .register_type::<Inventory::InventorySlot>()
            // Init events
            .add_event::<GameUI::InvSlotsBuild>()
            // Init Plugins
            .add_plugins(TextInputPlugin)
            // Init Systems ==========
            // LogoUI
            // MenuUI
            .add_systems(OnEnter(AppState::MainMenu), MenuUI::MainMenu::spawn_main_menu)
            .add_systems(Update, (
                MenuUI::MainMenu::interact_with_play_button.run_if(in_state(AppState::MainMenu)),
                MenuUI::MainMenu::interact_with_settings_button.run_if(in_state(AppState::MainMenu)),
                MenuUI::MainMenu::toggle_settings_window.run_if(in_state(AppState::MainMenu)),
                MenuUI::MainMenu::interact_with_quit_button.run_if(in_state(AppState::MainMenu))
            ))
            .add_systems(OnExit(AppState::MainMenu), MenuUI::MainMenu::despawn_main_menu)
            // GameUI
            .add_systems(OnEnter(AppState::Game), GameUI::GameUI::spawn_game_ui)
            .add_systems(Update, (
                GameUI::BarGui::build_gui.run_if(in_state(AppState::Game)),
                GameUI::BarGui::spawn_inventory_ui.run_if(in_state(AppState::Game)).after(GameUI::BarGui::build_gui),
                GameUI::BarGui::build_inv_slots.run_if(in_state(AppState::Game)).after(GameUI::BarGui::spawn_inventory_ui),
                GameUI::BarGui::update_inventory_ui.run_if(in_state(AppState::Game)),
                GameUI::BarGui::update_player_info.run_if(in_state(AppState::Game)),
                GameUI::GameUI::interact_with_to_menu_button.run_if(in_state(AppState::Game)),
                GameUI::GameUI::focus.run_if(in_state(AppState::Game))
            ))
            // GameUI === DEBUG
            .add_systems(Update, GameUI::DebugInfoPanel::toggle_debug_window.run_if(in_state(AppState::Game)))
            // GameUI === Console
            .add_systems(Update, Console::toggle_console.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), GameUI::GameUI::despawn_game_ui)
        ;
    }
}