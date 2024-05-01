use bevy::{
    app::AppExit,
    prelude::*, window::PresentMode
};
use bevy_egui::{
    egui,
    EguiContexts
};

use crate::core::{
    interface::Styles::*,
    Settings::Settings,
    AppState
};

// ==============================
// 
// ========== MainMenu ==========
#[derive(Component, Resource)]
pub struct MainMenu {
    pub settings_is_open: bool
}

// ========== Panel
// Панель для расположения кнопок
// ==========
#[derive(Component)]
pub struct MainMenuPanel {}

// ========== Button
// Кнопка буквально означает Играть
// ==========
#[derive(Component)]
pub struct PlayButton;

// ========== Button
// Кнопка для открытия окна настроек
// ==========
#[derive(Component)]
pub struct SettingsButton;

// ========== Panel
// Панель для расположения настроек
// ==========
// #[derive(Component)]
// pub struct SettingsWindow;

// ========== Button
// Кнопка для выхода
// ==========
#[derive(Component)]
pub struct QuitButton;

impl MainMenu {
    /// Функция для размещения Главного меню.
    pub fn spawn_main_menu(mut commands: Commands) {
        Self::build_main_menu(&mut commands);
    }

    /// Функция для создания элементов Главного меню.
    fn build_main_menu(commands: &mut Commands) {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                MainMenu {
                    settings_is_open: false
                },
                Name::new("Main Menu UI"),
            ))
            .with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            left: Val::Percent(80.0),
                            height: Val::Percent(100.0),
                            width: Val::Percent(20.0),
                            justify_self: JustifySelf::End,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: DARK_LGRAY_COLOR.into(),
                        ..default()
                    },
                    MainMenuPanel {},
                    Name::new("Main Menu Panel"),
                ))
                .with_children(|parent| {
                    // === Title ===
    
                    // === Play Button ===
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_container_style(75.0, 200.0),
                                border_color: Color::BLACK.into(),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            PlayButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Play",
                                        TextStyle {
                                            font_size: 28.0,
                                            ..default()
                                        },
                                    )],
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // === Settings Button ===
                    parent
                    .spawn((
                        ButtonBundle {
                            style: button_container_style(75.0, 200.0),
                            border_color: Color::BLACK.into(),
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        SettingsButton {},
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Settings",
                                    TextStyle {
                                        font_size: 28.0,
                                        ..default()
                                    },
                                )],
                                ..default()
                            },
                            ..default()
                        });
                    });
                    // === Quit Button ===
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_container_style(75.0, 200.0),
                                border_color: Color::BLACK.into(),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            QuitButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Quit",
                                        TextStyle {
                                            font_size: 28.0,
                                            ..default()
                                        },
                                    )],
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
            });
    }

    /// Функция для выгрузки Главного меню и его дочерних элементов.
    pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
        if let Ok(main_menu_entity) = main_menu_query.get_single() {
            commands.entity(main_menu_entity).despawn_recursive()
        }
    }

    // ==========
    // Функционал кнопки "Play"
    // ==========
    pub fn interact_with_play_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<PlayButton>),
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
                    app_state_next_state.set(AppState::LoadingInGame);
                    info!("State: LoadingInGame")
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

    // ========== SETTINGS
    // Функционал кнопки "Settings"
    // ==========
    pub fn interact_with_settings_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<SettingsButton>),
        >,
        mut menu: Query<&mut MainMenu, With<MainMenu>>
    ) {
        if button_query.is_empty() && menu.is_empty() {
            return;
        }

        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    if let Ok(mut menu) = menu.get_single_mut() {
                        menu.settings_is_open = !menu.settings_is_open;
                    }
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

    // ========== SETTINGS
    // Функционал переключения видимости Настроек
    // ==========
    pub fn toggle_settings_window(
        mut parent_query:   Query<&mut MainMenu, With<MainMenu>>,
        mut contexts:       EguiContexts,
        mut settings_res:   ResMut<Settings>,
        mut window:         Query<&mut Window>
    ) {
        if parent_query.is_empty() {
            return;
        }

        if let Ok(mut parent) = parent_query.get_single_mut() {
            if parent.settings_is_open {
                egui::Window::new("Settings")
                    .default_size((250.0, 300.0))
                    .show(contexts.ctx_mut(), |ui| {
                        ui.label("Settings");

                        ui.add(egui::Slider::new(&mut settings_res.rendering_distance, 0..=12)
                            .text("Loading range"));

                        if ui.checkbox(&mut settings_res.vsync, "VSync").changed() {
                            if let Ok(mut window) = window.get_single_mut() {
                                if settings_res.vsync {
                                    window.present_mode = PresentMode::AutoVsync
                                } else {
                                    window.present_mode = PresentMode::AutoNoVsync
                                }
                            }
                            
                        }

                        ui.horizontal(|ui| {
                            if ui.button("Apply").clicked() {
                                settings_res.save();
                            }
    
                            if ui.button("Reset").clicked() {
                                Settings::default().save();
                                *settings_res = Settings::load();
                            }

                            if ui.button("Готово").clicked() {
                                parent.settings_is_open = !parent.settings_is_open
                            }
                        })
                    });
            }
        }
    }

    // ==========
    // Функционал кнопки "Quit"
    // ==========
    pub fn interact_with_quit_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<QuitButton>),
        >,
        mut app_exit_event_writer: EventWriter<AppExit>,
    ) {
        if button_query.is_empty() {
            return;
        }

        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    app_exit_event_writer.send(AppExit);
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