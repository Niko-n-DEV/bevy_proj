#![allow(non_snake_case)]
pub mod game_ui;
// pub mod LogoUi;
pub mod MenuUI;
pub mod Styles;

use bevy::prelude::*;

use crate::core::AppState;
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Init resources
            .init_resource::<game_ui::Console::Console>()
            // Register Types (Inspector)
            .register_type::<game_ui::GameUI>()
            .register_type::<game_ui::Inventory::InventoryGui>()
            .register_type::<game_ui::Inventory::InventorySlot>()
            // Init events
            .add_event::<game_ui::Console::ConsoleInput>()
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
            .add_systems(OnEnter(AppState::Game), game_ui::GameUI::spawn_game_ui)
            .add_systems(Update, (
                    game_ui::BarGui::build_gui,
                    game_ui::BarGui::update_player_info,
                    game_ui::BarGui::interact_with_to_inv_visible_button,
                    game_ui::GameUI::interact_with_to_menu_button
                ).run_if(in_state(AppState::Game))
            )
            // GameUI === Info
            .add_systems(Update, 
                (
                game_ui::Info::info_item_panel,
                ).run_if(in_state(AppState::Game))
            )
            // GameUI === DEBUG
            .add_systems(Update, game_ui::DebugInfoPanel::toggle_debug_window.run_if(in_state(AppState::Game)))
            // GameUI === Console
            .add_systems(Update, (
                    game_ui::Console::toggle_console,
                    game_ui::Console::cmd_execute
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(OnExit(AppState::Game), game_ui::GameUI::despawn_game_ui)
        ;
    }
}