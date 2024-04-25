use bevy::{
    app::AppExit,
    prelude::*
};

use crate::core::{
    interface::Styles::*,
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

// ========== Event
// Событие нажатие на кнопку
// ==========
// #[derive(Event)]
// pub struct SettingsToggle;

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

// ==========
// Функционал кнопки "Settings"
// ==========
pub fn interact_with_settings_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SettingsButton>),
    >,
    //mut press_button: EventWriter<SettingsToggle>, 
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                //press_button.send(SettingsToggle);
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

// ==========
// Функционал кнопки "Quit"
// ==========
pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
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