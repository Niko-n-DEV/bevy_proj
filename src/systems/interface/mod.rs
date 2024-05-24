#![allow(non_snake_case)]
pub mod Console;
pub mod GameUI;
pub mod Info;
pub mod Inventory;
//pub mod LogoUi;
pub mod MenuUI;
pub mod Styles;
// pub mod UI;

use bevy::prelude::*;

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
            .add_event::<Console::ConsoleInput>()
            // Init Plugins
            // Init Systems ==========
            // LogoUI
            // MenuUI
            .add_systems(OnEnter(AppState::MainMenu), MenuUI::MainMenu::spawn_main_menu)
            .add_systems(Update, (
                    MenuUI::MainMenu::interact_with_play_button,
                    MenuUI::MainMenu::interact_with_settings_button,
                    MenuUI::MainMenu::toggle_settings_window,
                    MenuUI::MainMenu::interact_with_quit_button
                ).run_if(in_state(AppState::MainMenu))
            )
            .add_systems(OnExit(AppState::MainMenu), MenuUI::MainMenu::despawn_main_menu)
            // GameUI
            .add_systems(OnEnter(AppState::Game), GameUI::GameUI::spawn_game_ui)
            .add_systems(Update, (
                    GameUI::BarGui::build_gui,
                    GameUI::BarGui::update_player_info,
                    GameUI::BarGui::interact_with_to_inv_visible_button,
                    GameUI::GameUI::interact_with_to_menu_button
                ).run_if(in_state(AppState::Game))
            )
            // GameUI === Info
            .add_systems(Update, 
                (
                Info::info_item_panel,
                ).run_if(in_state(AppState::Game))
            )
            // GameUI === DEBUG
            .add_systems(Update, GameUI::DebugInfoPanel::toggle_debug_window.run_if(in_state(AppState::Game)))
            // GameUI === Console
            .add_systems(Update, (
                    Console::toggle_console,
                    Console::cmd_execute
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(OnExit(AppState::Game), GameUI::GameUI::despawn_game_ui)
        ;
    }
}