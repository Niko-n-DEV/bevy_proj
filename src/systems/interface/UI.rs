use bevy::app::AppExit;
use bevy::prelude::*;

use crate::core::{AppState, Styles::*};

// Main menu components =====
#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct QuitButton {}
// ==============================

// Game UI components =====
#[derive(Component)]
pub struct GameUI;
// ==============================

pub struct UI;

impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), Self::spawn_main_menu)
            .add_systems(Update, (transition_to_game_state, translation_to_main_menu))
            .add_systems(
                Update,
                (
                    Self::interact_with_play_button.run_if(in_state(AppState::MainMenu)),
                    Self::interact_with_quit_button.run_if(in_state(AppState::MainMenu)),
                ),
            )
            .add_systems(Update, exit_game)
            .add_systems(OnExit(AppState::MainMenu), Self::despawn_main_menu)
            .add_systems(OnEnter(AppState::Game), Self::spawn_game_ui)
            .add_systems(OnExit(AppState::Game), Self::despawn_game_ui);
    }
}

impl UI {
    /// Функция для размещения Главного меню.
    fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
        Self::build_main_menu(&mut commands, &asset_server);
    }

    /// Функция для создания элементов Главного меню.
    fn build_main_menu(commands: &mut Commands, _asset_server: &Res<AssetServer>) -> Entity {
        let main_menu_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Px(290.0),
                        justify_self: JustifySelf::End,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb_u8(40, 40, 40).into(),
                    ..default()
                },
                MainMenu {},
                Name::new("Main Menu UI")
            ))
            .with_children(|parent| {
                // === Title ===

                // === Play Button ===
                parent.spawn((
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
                                }
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
                // === Quit Button ===
                parent.spawn((
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
                                }
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
            })
            .id();
        main_menu_entity
    }

    /// Функция для выгрузки Главного меню и его дочерних элементов.
    fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
        if let Ok(main_menu_entity) = main_menu_query.get_single() {
            commands.entity(main_menu_entity).despawn_recursive()
        }
    }

    /// Функция для размещения игрового интерфейса.
    fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
        Self::build_game_ui(&mut commands, &asset_server);
    }

    /// Функция для размещения игрового интерфейса.
    fn build_game_ui(commands: &mut Commands, _asset_server: &Res<AssetServer>) -> Entity {
        let game_ui_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(5.0),
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::End,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                },
                GameUI {},
                Name::new("Game UI")
            ))
            .id();
        game_ui_entity
    }

    /// Функция для выгрузки игрового интерфейса и его дочерних элементов.
    fn despawn_game_ui(mut commands: Commands, game_ui_query: Query<Entity, With<GameUI>>) {
        if let Ok(game_ui_entity) = game_ui_query.get_single() {
            commands.entity(game_ui_entity).despawn_recursive();
        }
    }

    // === Для кнопок PlayButton и QuitButton
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
                    app_state_next_state.set(AppState::Game);
                    println!("Entered AppState::Game");
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
}

/// Функция для перехода на главную игровую сцену |
/// Функция для смения набора компонентов, меняя состояние приложение на `AppState::Game`
pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        if app_state.get() != &AppState::Game {
            app_state_next_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

/// Функция для перехода в Главное Меню |
/// Функция для смения набора компонентов, меняя состояние приложение на `AppState::MainMenu`
pub fn translation_to_main_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_reverse: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if app_state.get() != &AppState::MainMenu {
            app_state_reverse.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu")
        }
    }
}

/// Фунция выхода
pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::F4) {
        app_exit_event_writer.send(AppExit);
    }
}
