use bevy::prelude::*;
use bevy_egui::{
    egui,
    EguiContexts
};

use crate::core::{
    interface::{
        Styles::*,
        Inventory::*
    },
    world::World::WorldSystem,
    Entity::EntityBase,
    entities::EntitySystem::EnemySpawner,
    UserSystem::User,
    Container::Container,
    ItemType::*,
    AppState
};

// ==============================
// 
// ==========  GameUI  ==========
#[derive(Component, Resource)]
pub struct GameUI {
    pub bargui_is_open: bool,
    pub console_toggle: bool,
    pub debug_toggle: bool
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
                    debug_toggle:   false
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
            player:         Query<&EntityBase, With<User>>,
            keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if parent_query.is_empty() || player.is_empty() || spawners.is_empty() {
            return;
        }

        if let Ok(mut game_ui) = parent_query.get_single_mut() {
            if keyboard_input.just_pressed(KeyCode::F3) {
                game_ui.debug_toggle = !game_ui.debug_toggle
            }

            if game_ui.debug_toggle {
                let player_pos = player.single();

                egui::Window::new("Debug")
                    .show(contexts.ctx_mut(), |ui| {
                        ui.label("Position");
                        ui.vertical(|ui| {
                            ui.label(format!("Pos: {}", player_pos.position.0));
                            ui.label(format!("Pos_T: {}", WorldSystem::get_currect_chunk_tile(player_pos.position.0.truncate().as_ivec2())))
                        });

                        ui.horizontal(|ui| {
                            if ui.button("Spawners").clicked() {
                                for mut spawner in spawners.iter_mut() {
                                    spawner.is_active = !spawner.is_active
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
            user:       Query<&User>
    ) {
        if (game_ui.is_empty() && bar_gui.is_empty()) || user.is_empty() {
            return;
        }

        if let Ok(mut parent) = game_ui.get_single_mut() {
            let user = user.single();
            if !parent.1.bargui_is_open && user.control_entity != None {
                commands.entity(parent.0).with_children(|parent| {
                    // === BarGui ===
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                position_type:  PositionType::Absolute,
                                left:           Val::Percent(42.0),
                                bottom:         Val::Percent(0.0),
                                width:          Val::Percent(25.0),
                                height:         Val::Percent(14.0),
                                max_height:     Val::Px(100.0),
                                max_width:      Val::Px(300.0),
                                align_self:     AlignSelf::Center,
                                ..default()
                            },
                            background_color: DARK_LLGRAY_COLOR.into(),
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
                                    bottom: Val::Px(101.0),
                                    width:  Val::Px(125.0),
                                    height: Val::Px(15.0),
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(4.0)),
                                    ..default()
                                },
                                background_color: Color::rgb(0.18, 0.18, 0.18).into(),
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
                                    height: Val::Px(100.0),
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

    pub fn spawn_inventory_ui(
        mut commands:   Commands,
        mut bar_gui:    Query<(Entity, &mut BarGui), With<BarGui>>,
        mut event:      EventWriter<InvSlotsBuild>,
            game_ui:    Query<&GameUI, With<GameUI>>,
    ) {
        if bar_gui.is_empty() {
            return;
        }

        if let Ok(game_ui) = game_ui.get_single() {
            if game_ui.bargui_is_open {
                if let Ok(mut bar_gui) = bar_gui.get_single_mut() {
                    if !bar_gui.1.inventory_open {
                        commands.entity(bar_gui.0).with_children(|parent| {
                            parent.spawn((NodeBundle {
                                style: Style {
                                    display:    Display::Grid,
                                    left:       Val::Px(-176.0),
                                    bottom:     Val::Px(2.0),
                                    width:      Val::Px(136.0),
                                    height:     Val::Px(104.0),
                                    border: UiRect { 
                                        left:   Val::Px(4.), 
                                        right:  Val::Px(4.), 
                                        top:    Val::Px(4.), 
                                        bottom: Val::Px(4.) 
                                    },
                                    grid_template_columns: vec![GridTrack::px(32.), GridTrack::px(32.), GridTrack::px(32.), GridTrack::px(32.)],
                                    grid_template_rows: vec![
                                        GridTrack::px(32.),
                                        GridTrack::px(32.)
                                    ],
                                    ..default()
                                },
                                background_color:   Color::rgb(0.13, 0.13, 0.13).into(),
                                border_color:       Color::rgb(0.19, 0.19, 0.19).into(),
                                ..default()
                            },
                            InventoryGui {
                                slots: [InventorySlot::default(); 12]
                            },
                            Name::new("Inventory")
                            )).with_children(|slots| {
                                let mut slot_entities = Vec::new();
                                for i in 0..12 {
                                    let slot = slots.spawn(NodeBundle {
                                        style: Style {
                                            display: Display::Grid,
                                            border: UiRect { 
                                                left:   Val::Px(5.), 
                                                right:  Val::Px(5.), 
                                                top:    Val::Px(5.), 
                                                bottom: Val::Px(5.) 
                                            },
                                            aspect_ratio: Some(1.0),
                                            ..default()
                                        },
                                        background_color:   Color::rgb(0.24, 0.24, 0.24).into(),
                                        border_color:       Color::rgb(0.13, 0.13, 0.13).into(),
                                        ..default()
                                    })
                                    .insert(Name::new(format!("Slot {i}")))
                                    .id();
                                    
                                    slot_entities.push(slot);
                                }
                                event.send(InvSlotsBuild(slot_entities));
                            });
                        });
                        bar_gui.1.inventory_open = !bar_gui.1.inventory_open;
                    }
                }
            }
        }
    }

    pub fn build_inv_slots(
        mut inventory:  Query<(Entity, &mut InventoryGui), With<InventoryGui>>,
        mut event:      EventReader<InvSlotsBuild>
    ) {
        if event.is_empty() || inventory.is_empty() {
            return;
        }

        if let Ok(mut inv) = inventory.get_single_mut() {
            for event in event.read() {
                for (i, entity) in event.0.iter().enumerate() {
                    inv.1.slots[i] = InventorySlot::new(Some(i), Some(*entity))
                }
            }
        }
    }

    pub fn update_inventory_ui(
        mut commands:           Commands,
        mut inventoty_gui:      Query<(Entity, &mut InventoryGui), With<InventoryGui>>,
        mut inventoty:          Query<&mut Container, With<User>>
    ) {
        if inventoty_gui.is_empty() || inventoty.is_empty() {
            return;
        }

        if let Ok((_, mut inv_gui)) = inventoty_gui.get_single_mut() {
            if let Ok(mut inv_container) = inventoty.get_single_mut() {
                for i in 0..inv_gui.slots.len() {
                    if i >= inv_container.slots.len() {
                        break;
                    }
    
                    // Получаем слот инвентаря и его UI аналог
                    let inv_slot = &mut inv_container.slots[i];
                    let inv_gui_slot = &mut inv_gui.slots[i];
    
                    // Если слот инвентаря не пустой, обновляем информацию в UI
                    if inv_slot.item_stack.item_type != ItemType::None {
                        // Если слот UI уже содержит информацию о том же предмете, пропускаем обновление
                        if let Some(contain) = &inv_gui_slot.contain {
                            if contain.item_stack.item_type == inv_slot.item_stack.item_type {
                                continue;
                            }
                        }
    
                        // Очищаем дочерние элементы UI слота
                        if let Some(entity) = inv_gui_slot.entity {
                            commands.entity(entity).clear_children();
                        }
    
                        // Обновляем UI слота с новыми данными из инвентаря
                        if let Some(entity) = inv_gui_slot.entity {
                            commands.entity(entity).with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "TEST",
                                            TextStyle {
                                                font_size: 11.0,
                                                ..Default::default()
                                            },
                                        )],
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                            });
                        }
    
                        // Обновляем информацию UI слота инвентаря
                        inv_gui_slot.contain = Some(*inv_slot);
                    } else {
                        // Если слот инвентаря пустой, очищаем UI слота
                        if let Some(entity) = inv_gui_slot.entity {
                            commands.entity(entity).clear_children();
                        }
    
                        // Очищаем информацию UI слота инвентаря
                        inv_gui_slot.contain = None;
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
        mut inventory: Query<&mut Visibility, With<InventoryGui>>,
    ) {
        if button_query.is_empty() || inventory.is_empty() {
            return;
        }

        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    if let Ok(mut inventory) = inventory.get_single_mut() {
                        *inventory = match *inventory {
                            Visibility::Inherited => Visibility::Hidden,
                            Visibility::Hidden => Visibility::Visible,
                            Visibility::Visible => Visibility::Hidden,
                        }
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
        mut text_a:     Query<&mut Text, (With<AmmoBarGui>, (Without<HealthBarLine>, Without<HealthBarNum>))>,
            player:     Query<(&EntityBase, &Container), With<User>>
    ) {
        if (text_h.is_empty() && line_h.is_empty()) && text_a.is_empty() {
            return;
        }

        if let Ok(game_ui) = game_ui.get_single() {
            if game_ui.bargui_is_open {
                if let Ok(player) = player.get_single() {
                    if let Ok(mut text) = text_h.get_single_mut() {
                        let health = player.0.health.0;
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
            
                    if let Ok(mut text) = text_a.get_single_mut() {
                        if let Some(ammo) = player.1.find_in_container(ItemType::Item(Item::Ammo)) {
                            let count = ammo.item_stack.count;
                            text.sections = vec![TextSection::new(
                                format!("AMMO: {count}"),
                                TextStyle {
                                    font_size: 11.0,
                                    ..default()
                                },
                            )]
                        }
                    }
                }
            }
        }
    }
}


#[derive(Event)]
pub struct InvSlotsBuild(pub Vec<Entity>);
