//#![allow(unused)]
use bevy::{app::AppExit, prelude::*}; //, window::PrimaryWindow};

use bevy_simple_text_input::{TextInputBundle, TextInputInactive, TextInputPlugin};

use crate::core::{
    entities::EntitySystem::EnemySpawner,
    Entity::EntityBase,
    world::World::WorldSystem,
    UserSystem::{
        UserControl,
        User,
    },
    AppState, 
    interface::Styles::*
};

// Main menu components =====
#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct MainMenuPanel {}

#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct SettingsButton {}

#[derive(Event)]
pub struct SettingsToggle;

#[derive(Component)]
pub struct SettingsWindow;

#[derive(Component)]
pub struct QuitButton {}
// ==============================

// Game UI components =====
#[derive(Component)]
pub struct GameUI;

#[derive(Component, Resource)]
pub struct GameUIRes {
    pub debug_toggle: bool,
    pub settings_toggle: bool
}

impl Default for GameUIRes {
    fn default() -> Self {
        Self {
            debug_toggle: false,
            settings_toggle: false
        }
    }
}

#[derive(Component)]
pub struct CMDline;
// ==============================

// Debug UI components =====
#[derive(Component)]
pub struct DebugInfoPanel;

#[derive(Component)]
pub struct DebugPositionText;

#[derive(Component)]
pub struct DebugPositionTileText;

const BORDER_COLOR_ACTIVE: Color = Color::rgb(0.75, 0.52, 0.99);
const BORDER_COLOR_INACTIVE: Color = Color::rgb(0.25, 0.25, 0.25);
//const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
pub struct BackToMenuButton;

#[derive(Component)]
pub struct BarGui;
// ==============================

pub struct UI;

impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app
            // Init resources
            .init_resource::<GameUIRes>()
            // Init events
            .add_event::<SettingsToggle>()
            // Init Plugins
            .add_plugins(TextInputPlugin)
            // Init Systems
            .add_systems(OnEnter(AppState::MainMenu), Self::spawn_main_menu)
            .add_systems(
                Update,
                (
                    // In Menu
                    Self::interact_with_play_button.run_if(in_state(AppState::MainMenu)),
                    Self::interact_with_settings_button.run_if(in_state(AppState::MainMenu)),
                    Self::toggle_settings_window.run_if(in_state(AppState::MainMenu)),
                    Self::interact_with_quit_button.run_if(in_state(AppState::MainMenu)),
                    // In Game
                    Self::interact_with_to_menu_button.run_if(in_state(AppState::Game)),
                    Self::debug_toggle.run_if(in_state(AppState::Game)),
                    // In Game with Debug
                    interact_with_toggle_spawners_button.run_if(check_debug_toggle),
                    update_position_text.run_if(check_debug_toggle),
                    focus.run_if(in_state(AppState::Game)),
                ),
            )
            //.add_systems(Update, exit_game)
            //.add_systems(Update, (transition_to_game_state, translation_to_main_menu))
            .add_systems(OnExit(AppState::MainMenu), Self::despawn_main_menu)
            .add_systems(OnEnter(AppState::Game), Self::spawn_game_ui)
            .add_systems(OnExit(AppState::Game), Self::despawn_game_ui)
            ;
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
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                MainMenu {},
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
        let gameui_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.),
                        width: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
                GameUI,
                Interaction::None,
                Name::new("Game UI"),
            ))
            .with_children(|parent| {
                // === Base Node* ===
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(5.0),
                            align_items: AlignItems::Center,
                            align_self: AlignSelf::End,
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: DARK_LGRAY_COLOR.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // === Back To Menu Button ===
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_container_style(25.0, 45.0),
                                    border_color: Color::BLACK.into(),
                                    background_color: NORMAL_BUTTON_COLOR.into(),
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
                        // === CMDline ===
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                border_color: BORDER_COLOR_ACTIVE.into(),
                                background_color: BACKGROUND_COLOR.into(),
                                ..default()
                            },
                            Name::new("CMDline"),
                            TextInputBundle::default().with_inactive(true),
                            CMDline,
                        ));
                        // === BarGui ===
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(33.0),
                                    bottom: Val::Percent(0.0),
                                    width: Val::Percent(33.0),
                                    height: Val::Percent(200.0),
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            Name::new("BarGui"),
                            BarGui
                        ));
                    });
            })
            .id();
        gameui_entity
    }

    /// Функция для выгрузки игрового интерфейса и его дочерних элементов.
    fn despawn_game_ui(mut commands: Commands, game_ui_query: Query<Entity, With<GameUI>>) {
        if let Ok(game_ui_entity) = game_ui_query.get_single() {
            commands.entity(game_ui_entity).despawn_recursive();
        }
    }

    // === Для кнопок PlayButton и QuitButton в MainMenu
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

    pub fn interact_with_settings_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<SettingsButton>),
        >,
        mut press_button: EventWriter<SettingsToggle>, 
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    press_button.send(SettingsToggle);
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

    fn toggle_settings_window(
        mut commands: Commands,
        parent_query: Query<Entity, With<MainMenu>>,
        child_query: Query<Entity, With<SettingsWindow>>,
        mut parent_res: ResMut<GameUIRes>,
        event: EventReader<SettingsToggle>,
    ) {
        if event.is_empty() {
            return;
        }

        if !parent_res.settings_toggle {
            if let Ok(parent) = parent_query.get_single() {
                commands.entity(parent).with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(15.0),
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(50.0),
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                background_color: DARK_LLGRAY_COLOR.into(),
                                ..default() 
                            }, 
                            SettingsWindow
                        ))
                        .insert(Name::new("Settings"));
                });

                parent_res.settings_toggle = true;
            }
        } else {
            if let Ok(child) = child_query.get_single() {
                commands.entity(child).despawn_recursive();
                parent_res.settings_toggle = false;
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

    // === Для кнопок ... в GameUI
    pub fn interact_with_to_menu_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<BackToMenuButton>),
        >,
        mut app_state_next_state: ResMut<NextState<AppState>>,
    ) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    app_state_next_state.set(AppState::MainMenu);
                    info!("State: MainMenu")
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

    /// Функция для включения режим откладки
    pub fn debug_toggle(
        mut commands: Commands,
        parent_query: Query<Entity, With<GameUI>>,
        child_query: Query<Entity, With<DebugInfoPanel>>,
        mut parent_res: ResMut<GameUIRes>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_released(KeyCode::F5) {
            if !parent_res.debug_toggle {
                if let Ok(parent) = parent_query.get_single() {
                    commands.entity(parent).with_children(|parent| {
                        parent
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        width: Val::Percent(20.0),
                                        height: Val::Percent(75.0),
                                        align_self: AlignSelf::Center,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    background_color: DARK_LGRAY_COLOR.into(),
                                    ..default() 
                                }, 
                                DebugInfoPanel
                            ))
                            .with_children(|parent| {
                                // === Positon text ===
                                parent
                                .spawn((TextBundle {
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Pos:",
                                            TextStyle {
                                                font_size: 11.0,
                                                ..default()
                                            },
                                        )],
                                        ..default()
                                    },
                                    style: Style {
                                        display: Display::Grid,
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    ..default()
                                },
                                DebugPositionText
                                ));
                                // === Positon Tile text ===
                                parent
                                .spawn((TextBundle {
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "PosT:",
                                            TextStyle {
                                                font_size: 11.0,
                                                ..default()
                                            },
                                        )],
                                        ..default()
                                    },
                                    style: Style {
                                        display: Display::Grid,
                                        position_type: PositionType::Absolute,
                                        top: Val::Percent(5.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                DebugPositionTileText
                                ));
                                // === Button panel ===
                                parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        align_self: AlignSelf::End,
                                        ..default()
                                    },
                                    background_color: DARK_LLGRAY_COLOR.into(),
                                    ..default()
                                }).with_children(|parent| {
                                    // === Toggle spawners button ===
                                    parent
                                    .spawn((
                                        ButtonBundle {
                                            style: button_container_style(25.0, 60.0),
                                            border_color: Color::BLACK.into(),
                                            background_color: NORMAL_BUTTON_COLOR.into(),
                                            ..default()
                                        },
                                        ToggleSpawnersButton {},
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle {
                                            text: Text {
                                                sections: vec![TextSection::new(
                                                    "Spawner",
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
                            .insert(Name::new("Debug"));
                    });

                    parent_res.debug_toggle = true;
                }
            } else {
                if let Ok(child) = child_query.get_single() {
                    commands.entity(child).despawn_recursive();
                    parent_res.debug_toggle = false;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct ToggleSpawnersButton {}

fn check_debug_toggle(
    check_res: Res<GameUIRes>,
) -> bool {
    check_res.debug_toggle
}

pub fn interact_with_toggle_spawners_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ToggleSpawnersButton>),
    >,
    mut spawners: Query<(Entity, &mut EnemySpawner), With<EnemySpawner>>
) {
    if spawners.is_empty() && button_query.is_empty() {
        return;
    }

    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();

                for (_, mut spawner) in spawners.iter_mut() {
                    if spawner.is_active {
                        spawner.is_active = false
                    } else {
                        spawner.is_active = true
                    }
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

pub fn update_position_text(
    mut text: Query<&mut Text, (With<DebugPositionText>, Without<DebugPositionTileText>)>,
    mut text_t: Query<&mut Text, (With<DebugPositionTileText>, Without<DebugPositionText>)>,
    player: Query<&EntityBase, With<UserControl>>
) {
    if text.is_empty() || player.is_empty() {
        return;
    }

    let player_pos = player.single();

    if let Ok(mut text) = text.get_single_mut() {
        let (pos_x, pos_y) = (player_pos.position.0.x, player_pos.position.0.y);
        text.sections = vec![TextSection::new(
            format!("Pos: {pos_x} / {pos_y}"),
            TextStyle {
                font_size: 11.0,
                ..default()
            },
        )]
    }

    if let Ok(mut text_t) = text_t.get_single_mut() {
        let pos = WorldSystem::get_currect_chunk_tile(player_pos.position.0.truncate().as_ivec2());
        text_t.sections = vec![TextSection::new(
            format!("PosT: {} / {}", pos.x, pos.y),
            TextStyle {
                font_size: 11.0,
                ..default()
            },
        )]
    }
}

// /// Функция для перехода на главную игровую сцену |
// /// Функция для смения набора компонентов, меняя состояние приложение на `AppState::Game`
// pub fn transition_to_game_state(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     app_state: Res<State<AppState>>,
//     mut app_state_next_state: ResMut<NextState<AppState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Enter) {
//         if app_state.get() != &AppState::Game {
//             app_state_next_state.set(AppState::Game);
//             println!("Entered AppState::Game");
//         }
//     }
// }

// /// Функция для перехода в Главное Меню |
// /// Функция для смения набора компонентов, меняя состояние приложение на `AppState::MainMenu`
// pub fn translation_to_main_menu(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     app_state: Res<State<AppState>>,
//     mut app_state_reverse: ResMut<NextState<AppState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Escape) {
//         if app_state.get() != &AppState::MainMenu {
//             app_state_reverse.set(AppState::MainMenu);
//             println!("Entered AppState::MainMenu")
//         }
//     }
// }

/// focus для CMDline
fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = BORDER_COLOR_ACTIVE.into();
                } else {
                    inactive.0 = true;
                    *border_color = BORDER_COLOR_INACTIVE.into();
                }
            }
        }
    }
}

// /// Фунция выхода
// pub fn exit_game(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut app_exit_event_writer: EventWriter<AppExit>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::F4) {
//         app_exit_event_writer.send(AppExit);
//     }
// }