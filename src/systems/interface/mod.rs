#![allow(non_snake_case)]
pub mod GameUI;
pub mod LogoUi;
pub mod MenuUI;
pub mod Styles;
pub mod UI;

use bevy::prelude::*;

use bevy_simple_text_input::TextInputPlugin;

use crate::core::AppState;

// Расфокусить Plugin UI на несколько и чётко направленных местах. Menu в меню, Game UI в Game и т.п.

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Init resources
            .init_resource::<GameUI::GameUIRes>()
            // Init events
            // Init Plugins
            .add_plugins(TextInputPlugin)
            // Init Systems ==========
            // LogoUI
            // MenuUI
            .add_systems(OnEnter(AppState::MainMenu), MenuUI::MainMenu::spawn_main_menu)
            .add_systems(Update, (
                MenuUI::interact_with_play_button.run_if(in_state(AppState::MainMenu)),
                MenuUI::interact_with_settings_button.run_if(in_state(AppState::MainMenu)),
                MenuUI::interact_with_quit_button.run_if(in_state(AppState::MainMenu))
            ))
            .add_systems(OnExit(AppState::MainMenu), MenuUI::MainMenu::despawn_main_menu)
            // GameUI
            .add_systems(OnEnter(AppState::Game), GameUI::GameUI::spawn_game_ui)
            .add_systems(Update, (
                GameUI::interact_with_to_menu_button.run_if(in_state(AppState::Game)),
                GameUI::focus.run_if(in_state(AppState::Game))
            ))
            .add_systems(Update, (
                GameUI::debug_toggle.run_if(in_state(AppState::Game)),
                GameUI::update_position_text.run_if(GameUI::check_debug_toggle),
                GameUI::interact_with_toggle_spawners_button.run_if(GameUI::check_debug_toggle)
            ))
            .add_systems(OnExit(AppState::Game), GameUI::GameUI::despawn_game_ui)
        ;
    }
}