use bevy::prelude::*;

use crate::core::{
    interface::Styles::*,
    Entity::EntityBase,
    resource::graphic::Atlas::{
        AtlasRes, 
        AtlasType
    }, 
    ContainerSystem::Inventory as Container,
    UserSystem::{
        User, 
        UserControl
    },
    AppState
};

use super::{GameUI, Inventory};


pub fn bargui_ui_plugin(app: &mut App) {
    app.add_systems(Update, 
        (
            BarGui::build_gui,
            BarGui::update_player_info,
            BarGui::interact_with_to_about_avatar_button,
            BarGui::interact_with_to_inv_visible_button,
            BarGui::interact_with_to_handle_crafting_button
        ).run_if(in_state(AppState::Game))
    );
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

// === Bar Indicator

#[derive(Component)]
pub struct HealthBarLine;

#[derive(Component)]
pub struct HealthBarNum;

#[allow(unused)]
#[derive(Component)]
pub struct AmmoBarGui;

// === Option Panel

#[derive(Component)]
pub struct AboutAvatarButton;

#[derive(Component)]
pub struct ToggleInvVisibleButton;

#[derive(Component)]
pub struct HandleCraftingButton;

impl BarGui {
    /// Функция для создания пользовательского интерфейса
    /// 
    /// Для имеющигося под контролем пользовательского юнита
    pub fn build_gui(
        mut commands:   Commands,
        mut game_ui:    Query<(Entity, &mut GameUI), (With<GameUI>, Without<BarGui>)>,
            bar_gui:    Query<Entity, (With<BarGui>, Without<GameUI>)>,
            atlas:      Res<AtlasRes>,
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
                                border:         UiRect::all(Val::Px(3.0)),
                                ..default()
                            },
                            background_color:   BASE_UI_COLOR.into(),
                            border_color:       BASE_BORDER_UI_COLOR.into(),
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
                                    ..default()
                                },
                                background_color:   WIDGET_UI_COLOR.into(),
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
                                    background_color: HEALTH_COLOR.into(),
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

                        // parent.spawn((
                        //     TextBundle {
                        //         style: Style {
                        //             position_type:  PositionType::Absolute,
                        //             top:            Val::Percent(10.0),
                        //             ..default()
                        //         },
                        //         text: Text {
                        //             sections: vec![TextSection::new(
                        //                 "AMMO:",
                        //                 TextStyle {
                        //                     font_size: 11.0,
                        //                     ..default()
                        //                 },
                        //             )],
                        //             ..default()
                        //         },
                        //         ..default()
                        //     },
                        //     AmmoBarGui
                        // ));

                        // === Options Bar

                        parent.spawn(
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    display: Display::Grid,
                                    left:   Val::Px(-43.0),
                                    bottom: Val::Px(-3.0),
                                    width:  Val::Px(40.0),
                                    height: Val::Px(104.0),
                                    ..default()
                                },
                                background_color:   BASE_EX_UI_COLOR.into(),
                                ..default()
                            }
                        ).with_children(|parent| {

                            // == AboutAvatar

                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        height:             Val::Px(30.0),
                                        width:              Val::Px(30.0),
                                        justify_content:    JustifyContent::Center,
                                        align_items:        AlignItems::Center,
                                        border:             UiRect::all(Val::Px(2.0)),
                                        margin:             UiRect {
                                            left:   Val::Px(5.0),
                                            right:  Val::Px(5.0),
                                            top:    Val::Px(4.0),
                                            bottom: Val::Px(1.5)
                                        },
                                        ..default()
                                    },
                                    border_color:       BTN_BORDER_COLOR.into(),
                                    background_color:   BTN_COLOR.into(),
                                    ..default()
                                },
                                AboutAvatarButton
                            )).with_children(|parent| {
                                if let Some(img) = atlas.get_texture(AtlasType::Ui, "about_avatar_ui_btn") {
                                    parent.spawn((
                                        ImageBundle {
                                            style: Style {
                                                height: Val::Percent(100.0),
                                                width:  Val::Percent(100.0),
                                                ..default()
                                            },
                                            image: UiImage::new(img.1),
                                            ..default()
                                        },
                                        img.0
                                    ));
                                } else {
                                    warn!("Текстура не применена - Необходимая текстура не найдена!")
                                }
                            });

                            // == ToggleInvVisibleButton

                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        height:             Val::Px(30.0),
                                        width:              Val::Px(30.0),
                                        justify_content:    JustifyContent::Center,
                                        align_items:        AlignItems::Center,
                                        border:             UiRect::all(Val::Px(2.0)),
                                        margin:             UiRect {
                                            left:   Val::Px(5.0),
                                            right:  Val::Px(5.0),
                                            top:    Val::Px(1.5),
                                            bottom: Val::Px(1.5)
                                        },
                                        ..default()
                                    },
                                    border_color:       BTN_BORDER_COLOR.into(),
                                    background_color:   BTN_COLOR.into(),
                                    ..default()
                                },
                                ToggleInvVisibleButton
                            )).with_children(|parent| {
                                if let Some(img) = atlas.get_texture(AtlasType::Ui, "inv_ui_btn") {
                                    parent.spawn((
                                        ImageBundle {
                                            style: Style {
                                                height: Val::Percent(100.0),
                                                width:  Val::Percent(100.0),
                                                ..default()
                                            },
                                            image: UiImage::new(img.1),
                                            ..default()
                                        },
                                        img.0
                                    ));
                                } else {
                                    warn!("Текстура не применена - Необходимая текстура не найдена!")
                                }
                            });

                            // == Hand crafting

                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        height:             Val::Px(30.0),
                                        width:              Val::Px(30.0),
                                        justify_content:    JustifyContent::Center,
                                        align_items:        AlignItems::Center,
                                        border:             UiRect::all(Val::Px(2.0)),
                                        margin:             UiRect {
                                            left:   Val::Px(5.0),
                                            right:  Val::Px(5.0),
                                            top:    Val::Px(1.5),
                                            bottom: Val::Px(4.0)
                                        },
                                        ..default()
                                    },
                                    border_color:       BTN_BORDER_COLOR.into(),
                                    background_color:   BTN_COLOR.into(),
                                    ..default()
                                },
                                HandleCraftingButton
                            )).with_children(|parent| {
                                if let Some(img) = atlas.get_texture(AtlasType::Ui, "crafting_ui_btn") {
                                    parent.spawn((
                                        ImageBundle {
                                            style: Style {
                                                height: Val::Percent(100.0),
                                                width:  Val::Percent(100.0),
                                                ..default()
                                            },
                                            image: UiImage::new(img.1),
                                            ..default()
                                        },
                                        img.0
                                    ));
                                } else {
                                    warn!("Текстура не применена - Необходимая текстура не найдена!")
                                }
                            });
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

    pub fn interact_with_to_about_avatar_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<AboutAvatarButton>),
        >
    ) {
        if button_query.is_empty() {
            return;
        }

        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = BTN_PRESS_COLOR.into();
                }
                Interaction::Hovered => {
                    *background_color = BTN_HOVER_COLOR.into();
                }
                Interaction::None => {
                    *background_color = BTN_COLOR.into();
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
                    *background_color = BTN_PRESS_COLOR.into();
                    
                    if let Ok(player) = player.get_single() {
                        inventory_toggle_writer.send(Inventory::InventoryDisplayToggleEvent { actor: player });
                    }
                }
                Interaction::Hovered => {
                    *background_color = BTN_HOVER_COLOR.into();
                }
                Interaction::None => {
                    *background_color = BTN_COLOR.into();
                }
            }
        }
    }

    pub fn interact_with_to_handle_crafting_button(
        mut button_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<HandleCraftingButton>),
        >
    ) {
        if button_query.is_empty() {
            return;
        }

        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *background_color = BTN_PRESS_COLOR.into();
                }
                Interaction::Hovered => {
                    *background_color = BTN_HOVER_COLOR.into();
                }
                Interaction::None => {
                    *background_color = BTN_COLOR.into();
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
            
                    // Тут должен быть счётчик боеприпасов
                }
            }
        }
    }
}