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
            // Game UI
            .add_plugins(game_ui::game_ui_plugin)
        ;
    }
}