pub mod BarGui;
pub mod Console;
pub mod Context;
pub mod Debug;
pub mod Info;
pub mod Inventory;
pub mod Select;

use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::AppState;

use super::Styles::*;

// ==============================
//             Module
// ==========  GameUI  ==========

pub fn game_ui_plugin(app: &mut App) {
    // GameUI
    app.add_systems(OnEnter(AppState::Game), GameUI::spawn_game_ui);
    app.add_systems(Update, GameUI::interact_with_to_menu_button.run_if(in_state(AppState::Game)));
    // GameUI === BarGui
    app.add_plugins(BarGui::bargui_ui_plugin);
    // GameUI === Info
    app.add_systems(Update, 
        (
        Info::info_item_panel,
        ).run_if(in_state(AppState::Game))
    );
    // GameUI === Selector
    app.add_plugins(Select::select_plugin);
    // GameUI === DEBUG
    app.add_plugins(Debug::debug_ui_plugin);
    // GameUI === Console
    app.add_plugins(Console::console_plugin);
    // GameUI === ContextMenu
    app.add_plugins(Context::context_menu_plugin);
    app.add_systems(OnExit(AppState::Game), GameUI::despawn_game_ui);
}

// ==============================
// 
// ==========  GameUI  ==========
#[derive(Component, InspectorOptions, Reflect, Resource)]
#[reflect(Component, InspectorOptions)]
pub struct GameUI {
    pub bargui_is_open: bool,
    pub console_toggle: bool,
    pub debug_toggle:   bool,
    pub debug_menu:     bool
}

#[allow(unused)]
enum DebugMode {
    Active,
    NonActive
}

// ========== Button
// Кнопка для открытия меню
// ==========
#[derive(Component)]
pub struct BackToMenuButton;

impl GameUI {
    /// Функция для размещения игрового интерфейса.
    pub fn spawn_game_ui(
        mut commands: Commands, 
        asset_server: Res<AssetServer>
    ) {
        Self::build_game_ui(&mut commands, &asset_server);
    }

    /// Функция для размещения игрового интерфейса.
    fn build_game_ui(
        commands:       &mut Commands, 
        _asset_server:  &Res<AssetServer>
    ) -> Entity {
        let gameui_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.),
                        width:  Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
                GameUI {
                    bargui_is_open: false,
                    console_toggle: false,
                    debug_toggle:   false,
                    debug_menu:     false
                },
                Interaction::None,
                Name::new("Game UI"),
            ))
            .with_children(|parent| {

                // === Base Node* ===

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width:          Val::Percent(100.0),
                            height:         Val::Percent(5.0),
                            align_items:    AlignItems::Center,
                            align_self:     AlignSelf::End,
                            padding:        UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: BASE_LINE_UI_COLOR.into(),
                        ..default()
                    })
                    .with_children(|parent| {

                        // === Back To Menu Button ===

                        parent
                            .spawn((
                                ButtonBundle {
                                    style:              button_container_style(25.0, 45.0),
                                    border_color:       Color::BLACK.into(),
                                    background_color:   NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                BackToMenuButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Menu",
                                            TextStyle {
                                                font_size: 11.0,
                                                ..default()
                                            },
                                        )],
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    });
            })
            .id();
        gameui_entity
    }

    /// Функция для выгрузки игрового интерфейса и его дочерних элементов.
    pub fn despawn_game_ui(mut commands: Commands, game_ui_query: Query<Entity, With<GameUI>>) {
        if let Ok(game_ui_entity) = game_ui_query.get_single() {
            commands.entity(game_ui_entity).despawn_recursive();
        }
    }

    // ==========
    // Функционал кнопки "Menu"
    // ==========
    pub fn interact_with_to_menu_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<BackToMenuButton>),
        >,
        mut app_state_next_state: ResMut<NextState<AppState>>,
    ) {
        if button_query.is_empty() {
            return;
        }

        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    app_state_next_state.set(AppState::MainMenu);
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = NORMAL_BUTTON_COLOR.into();
                }
            }
        }
    }
}