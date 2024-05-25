pub mod Console;
pub mod Info;
pub mod Inventory;

use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use bevy_egui::{
    egui,
    EguiContexts
};

use crate::core::{
    interface::Styles::*,
    world::World::WorldSystem,
    Entity::EntityBase,
    entities::EntitySystem::EnemySpawner,
    UserSystem::{
        User,
        UserControl,
        CursorPlacer
    },
    ContainerSystem::Inventory as Container,
    resource::Registry::Registry,
    AppState
};

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
                        background_color: DARK_LGRAY_COLOR.into(),
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

// ========== DEBUG ==========
pub struct DebugInfoPanel;

impl DebugInfoPanel {
    // ==========
    // Функционал для открытия дебага
    // ==========
    pub fn toggle_debug_window(
        mut parent_query:   Query<&mut GameUI, With<GameUI>>,
        mut contexts:       EguiContexts,
        mut spawners:       Query<&mut EnemySpawner, With<EnemySpawner>>,
            registry:       Res<Registry>,
        mut placer:         ResMut<CursorPlacer>,
            player:         Query<&EntityBase, With<UserControl>>,
            keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if parent_query.is_empty() {
            return;
        }

        if let Ok(mut game_ui) = parent_query.get_single_mut() {
            if keyboard_input.just_pressed(KeyCode::F3) {
                game_ui.debug_menu = false;
                game_ui.debug_toggle = !game_ui.debug_toggle
            }

            if game_ui.debug_toggle {
                egui::Window::new("Debug")
                    .show(contexts.ctx_mut(), |ui| {
                        ui.label("Position");
                        if let Ok(player_pos) = player.get_single() {
                            ui.vertical(|ui| {
                                ui.label(format!("Pos: {}", player_pos.position.0));
                                ui.label(format!("Pos_T: {}", WorldSystem::get_currect_chunk_tile(player_pos.position.0.as_ivec2())))
                            });
                        }

                        ui.horizontal(|ui| {
                            if ui.button("Spawners").clicked() && !spawners.is_empty() {
                                for mut spawner in spawners.iter_mut() {
                                    spawner.is_active = !spawner.is_active
                                }
                            }
                        });

                        if ui.button("menu").clicked() {
                            game_ui.debug_menu = !game_ui.debug_menu;
                        }
                    });
            }

            if game_ui.debug_menu {
                egui::Window::new("Debug Menu")
                    .show(contexts.ctx_mut(), |ui| {
                        ui.label("Items");
                        ui.horizontal(|ui| {
                            for key in registry.item_registry.keys() {
                                if ui.button(key).clicked() {
                                    placer.placer = Some(("item".to_string(), key.clone()));
                                }
                            }
                        });

                        ui.label("Objects");
                        ui.horizontal(|ui| {
                            for key in registry.object_registry.keys() {
                                if ui.button(key).clicked() {
                                    placer.placer = Some(("object".to_string(), key.clone()));
                                }
                            }
                        });

                        ui.label("Entities");
                        ui.horizontal(|ui| {
                            for key in registry.entity_registry.keys() {
                                if ui.button(key).clicked() {
                                    placer.placer = Some(("entity".to_string(), key.clone()));
                                }
                            }
                        });
                    });
                
            }
        }
    }
}

// ==============================
// 
// ==========  BARGUI  ==========

// ========== Panel
// Панель пользовательского интерфейса для отображения информации и быстрого взаимодействия с персонажем
// ==========
#[derive(Component)]
pub struct BarGui {
    pub inventory_open: bool
}

#[derive(Component)]
pub struct HealthBarLine;

#[derive(Component)]
pub struct HealthBarNum;

#[derive(Component)]
pub struct AmmoBarGui;

#[derive(Component)]
pub struct ToggleInvVisibleButton;

impl BarGui {
    /// Функция для создания пользовательского интерфейса
    /// 
    /// Для имеющигося под контролем пользовательского юнита
    pub fn build_gui(
        mut commands:   Commands,
        mut game_ui:    Query<(Entity, &mut GameUI), (With<GameUI>, Without<BarGui>)>,
            bar_gui:    Query<Entity, (With<BarGui>, Without<GameUI>)>,
            user:       Res<User>
    ) {
        if game_ui.is_empty() && bar_gui.is_empty() {
            return;
        }

        if let Ok(mut parent) = game_ui.get_single_mut() {
            if !parent.1.bargui_is_open && !user.control_entity.is_none() {
                commands.entity(parent.0).with_children(|parent| {
                    // === BarGui ===
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                position_type:  PositionType::Absolute,
                                left:           Val::Percent(42.0),
                                bottom:         Val::Percent(0.0),
                                width:          Val::Percent(25.0),
                                height:         Val::Percent(15.0),
                                max_height:     Val::Px(104.0),
                                max_width:      Val::Px(300.0),
                                align_self:     AlignSelf::Center,
                                border: UiRect {
                                    right:  Val::Px(3.),
                                    ..default()
                                },
                                ..default()
                            },
                            background_color:   Color::rgb(0.15, 0.15, 0.15).into(),
                            border_color:       Color::rgb(0.19, 0.19, 0.19).into(),
                            ..default()
                        },
                        Name::new("BarGui"),
                        BarGui {
                            inventory_open: false
                        }
                    )).with_children(|parent| {
                        // === Health Bar
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    bottom: Val::Px(108.5),
                                    width:  Val::Px(125.0),
                                    height: Val::Px(15.0),
                                    align_items:    AlignItems::Center,
                                    padding:        UiRect::all(Val::Px(4.0)),
                                    border:         UiRect { 
                                        left:   Val::Px(1.), 
                                        right:  Val::Px(1.), 
                                        top:    Val::Px(1.), 
                                        bottom: Val::Px(1.) 
                                    },
                                    ..default()
                                },
                                background_color:   Color::rgb(0.18, 0.18, 0.18).into(),
                                border_color:       Color::rgb(0.20, 0.20, 0.20).into(),
                                ..default()
                            },
                            Name::new("HealthBar")
                        )).with_children(|parent| {
                            // === Line Health
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        height: Val::Px(7.0),
                                        width:  Val::Px(90.0),
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.79, 0.15, 0.15).into(),
                                    ..default()
                                },
                                HealthBarLine
                            ));

                            // === Text Health
                            parent.spawn((
                                TextBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        left: Val::Percent(80.0),
                                        ..default()
                                    },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "%",
                                            TextStyle {
                                                font_size: 11.0,
                                                ..default()
                                            },
                                        )],
                                        ..default()
                                    },
                                    ..default()
                                },
                                HealthBarNum
                            ));
                        });

                        // === Ammo Bar
                        parent.spawn((
                            TextBundle {
                                style: Style {
                                    position_type:  PositionType::Absolute,
                                    top:            Val::Percent(10.0),
                                    ..default()
                                },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "AMMO:",
                                        TextStyle {
                                            font_size: 11.0,
                                            ..default()
                                        },
                                    )],
                                    ..default()
                                },
                                ..default()
                            },
                            AmmoBarGui
                        ));

                        // === Options Bar
                        parent.spawn(
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    left:   Val::Px(-40.0),
                                    width:  Val::Px(40.0),
                                    height: Val::Px(104.0),
                                    border: UiRect { 
                                        left:   Val::Px(2.), 
                                        right:  Val::Px(2.), 
                                        top:    Val::Px(2.), 
                                        bottom: Val::Px(2.) 
                                    },
                                    ..default()
                                },
                                background_color:   Color::rgb(0.25, 0.25, 0.25).into(),
                                border_color:       Color::rgb(0.30, 0.30, 0.30).into(),
                                ..default()
                            }
                        ).with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    style:              button_container_style(25.0, 45.0),
                                    border_color:       Color::rgb(0.20, 0.20, 0.20).into(),
                                    background_color:   Color::rgb(0.35, 0.35, 0.35).into(),
                                    ..default()
                                },
                                ToggleInvVisibleButton
                            ));
                        });
                    });
                });
                parent.1.bargui_is_open = true;
            } else {
                if user.control_entity == None {
                    if let Ok(child) = bar_gui.get_single() {
                        commands.entity(child).despawn_recursive();
                        parent.1.bargui_is_open = false;
                    }
                }
            }
        }
    }

    pub fn interact_with_to_inv_visible_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<ToggleInvVisibleButton>),
        >,
        player: Query<
            Entity,
            (
                With<UserControl>,
                With<Container>,
            ),
        >,
        mut inventory_toggle_writer: EventWriter<Inventory::InventoryDisplayToggleEvent>,
    ) {
        if button_query.is_empty() {
            return;
        }

        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    if let Ok(player) = player.get_single() {
                        inventory_toggle_writer.send(Inventory::InventoryDisplayToggleEvent { actor: player });
                    }
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = Color::rgb(0.35, 0.35, 0.35).into();
                }
            }
        }
    }

    // ========== BARGUI
    // Обновление информации о здоровье и кол-ва патронов в инвентаре
    // ==========
    pub fn update_player_info(
            game_ui:    Query<&GameUI, With<GameUI>>,
        mut line_h:     Query<&mut Style, (With<HealthBarLine>, Without<HealthBarNum>)>,
        mut text_h:     Query<&mut Text, (With<HealthBarNum>, Without<HealthBarLine>)>,
        // mut text_a:     Query<&mut Text, (With<AmmoBarGui>, (Without<HealthBarLine>, Without<HealthBarNum>))>,
        //    player:     Query<(&EntityBase, &Container), With<User>>
            player:     Query<&EntityBase, With<UserControl>>
    ) {
        if text_h.is_empty() && line_h.is_empty() { // && text_a.is_empty() {
            return;
        }

        if let Ok(game_ui) = game_ui.get_single() {
            if game_ui.bargui_is_open {
                if let Ok(player) = player.get_single() {
                    if let Ok(mut text) = text_h.get_single_mut() {
                        let health = player.health.0;
                        text.sections = vec![TextSection::new(
                            format!("{health}%"),
                            TextStyle {
                                font_size: 11.0,
                                ..default()
                            },
                        )];
                        
                        if let Ok(mut line) = line_h.get_single_mut() {
                            line.width = Val::Px(health * 0.9)
                        }
                    }
            
                    // if let Ok(mut text) = text_a.get_single_mut() {
                    //     if let Some(ammo) = player.1.find_in_container(ItemType::Item(Item::Ammo)) {
                    //         let count = ammo.item_stack.count;
                    //         text.sections = vec![TextSection::new(
                    //             format!("AMMO: {count}"),
                    //             TextStyle {
                    //                 font_size: 11.0,
                    //                 ..default()
                    //             },
                    //         )]
                    //     }
                    // }
                }
            }
        }
    }
}
