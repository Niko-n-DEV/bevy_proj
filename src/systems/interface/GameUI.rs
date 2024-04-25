use bevy::prelude::*;

use crate::core::{
    interface::Styles::*,
    world::World::WorldSystem,
    Entity::EntityBase,
    entities::EntitySystem::EnemySpawner,
    UserSystem::User,
    AppState
};

use bevy_simple_text_input::{TextInputBundle, TextInputInactive};

// ==============================
// 
// ==========  GameUI  ==========
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

// ========== Button
// Кнопка для открытия меню
// ==========
#[derive(Component)]
pub struct BackToMenuButton;

const BORDER_COLOR_ACTIVE: Color = Color::rgb(0.75, 0.52, 0.99);
const BORDER_COLOR_INACTIVE: Color = Color::rgb(0.25, 0.25, 0.25);
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

// ========== TextLine
// Поле ввода
// ==========
#[derive(Component)]
pub struct CMDline;

// ========== Panel
// Панель пользовательского интерфейса для отображения информации и быстрого взаимодействия с персонажем
// ==========
#[derive(Component)]
pub struct BarGui;

// ========== DEBUG ==========
// ========== Panel
// Панель дабага
// ==========
#[derive(Component)]
pub struct DebugInfoPanel;

// ========== Text
// Текст для отображения информации о местоположении по точным координатам
// ==========
#[derive(Component)]
pub struct DebugPositionText;

// ========== Text
// Текст для отображения информации о местоположении по фиксировано по клеткам координат
// ==========
#[derive(Component)]
pub struct DebugPositionTileText;

// ========== Button
// Кнопка для переключения спавн поинтов
// ==========
#[derive(Component)]
pub struct ToggleSpawnersButton {}

impl GameUI {
    /// Функция для размещения игрового интерфейса.
    pub fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    pub fn despawn_game_ui(mut commands: Commands, game_ui_query: Query<Entity, With<GameUI>>) {
        if let Ok(game_ui_entity) = game_ui_query.get_single() {
            commands.entity(game_ui_entity).despawn_recursive();
        }
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

// ==========
// Функционал фокуса для текстовой линии
// ==========
/// focus для CMDline
pub fn focus(
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

// ==========
// Функционал для открытия дебага
// ==========

/// Проверка, включен ли дебаг мод
pub fn check_debug_toggle(
    check_res: Res<GameUIRes>,
) -> bool {
    check_res.debug_toggle
}

/// Функция для включения режим откладки
pub fn debug_toggle(
    mut commands: Commands,
    parent_query: Query<Entity, With<GameUI>>,
    mut parent_res: ResMut<GameUIRes>,
    child_query: Query<Entity, With<DebugInfoPanel>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        if let Ok(parent) = parent_query.get_single() {
            if !parent_res.debug_toggle {
                println!("да");
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
            } else {
                println!("нет");
                if let Ok(child) = child_query.get_single() {
                    commands.entity(child).despawn_recursive();
                    parent_res.debug_toggle = false;
                }
            }
        }
        
    }
}

// ==========
// Обновление данных о местоположении игрока
// ==========
pub fn update_position_text(
    mut text: Query<&mut Text, (With<DebugPositionText>, Without<DebugPositionTileText>)>,
    mut text_t: Query<&mut Text, (With<DebugPositionTileText>, Without<DebugPositionText>)>,
    player: Query<&EntityBase, With<User>>
) {
    if text.is_empty() || player.is_empty() {
        println!("skip");
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

// ==========
// Функционал переключения спавн поинтов
// ==========
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
