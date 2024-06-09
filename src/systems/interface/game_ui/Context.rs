use bevy::{
    prelude::*, 
    window::PrimaryWindow
};

use crate::core::{
    Item::TakeItem,
    interface::game_ui::Select::Selected, 
    UserSystem::UserControl,
    Camera::UserCamera,
    ContainerSystem::Slot,
    AppState,
};

pub fn context_menu_plugin(app: &mut App) {
    // Init res
    app.init_resource::<ContextUIPos>();
    app.init_resource::<ContextMenuRes>();
    // Init systems
    app.add_systems(PreUpdate, track_pos_context.run_if(in_state(AppState::Game)));
    app.add_systems(Update, 
        (
                interact_with_about_context_button,
                interact_with_take_context_button,
                track_context.after(track_pos_context)
            ).run_if(in_state(AppState::Game))
        );
    app.add_systems(PostUpdate, def_context_menu.run_if(in_state(AppState::Game)));
}

#[derive(Resource, Default)]
struct ContextUIPos {
    pub procent_pos: Vec2
}

// ==============================
//  Context Menu
//  Builder
// ==============================

fn build_context_menu(
    commands:   &mut Commands,
) -> Entity {
    commands.spawn((
        NodeBundle {
            style: Style {
                display:    Display::Grid,
                height:     Val::Percent(10.0),
                width:      Val::Percent(10.0),
                border: UiRect {
                    left:   Val::Percent(0.5),
                    right:  Val::Percent(0.5),
                    top:    Val::Percent(0.5),
                    bottom: Val::Percent(0.5),
                },
                ..default()
            },
            background_color:   Color::rgb(0.19, 0.19, 0.19).into(),
            border_color:       Color::rgb(0.25, 0.25, 0.25).into(),
            ..default()
        },
        ContextMenu,
        Name::new("ContextMenu")
    )).with_children(|parent| {

        // === AboutButton

        parent.spawn((
            ButtonBundle {
                style: Style {
                    display:    Display::Grid,
                    // height: Val::Percent(25.),
                    // width:  Val::Percent(100.),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color:   Color::rgb(0.22, 0.22, 0.22).into(),
                ..default()
            },
            AboutButton,
            Name::new("AboutBtn")
        )).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "About",
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

        // === Take

        parent.spawn((
            ButtonBundle {
                style: Style {
                    display:    Display::Grid,
                    // height: Val::Percent(25.),
                    // width:  Val::Percent(100.),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color:   Color::rgb(0.21, 0.21, 0.21).into(),
                ..default()
            },
            TakeButton,
            Name::new("TaketBtn")
        )).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Take",
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

        // === Atack


    }).id()
}


// ==============================
//  Context Menu
//  Context Objects
// ==============================

#[derive(Component)]
struct ContextMenu;

#[allow(unused)]
#[derive(Resource, Default)]
struct ContextMenuRes {
    pub is_enable:  bool,
    pub slot:       Option<Slot>,
    pub object:     Option<Entity>
}

fn track_pos_context(
    mut context_res:    ResMut<ContextUIPos>,
        window:         Query<&Window, With<PrimaryWindow>>,
        camera:         Query<(&Camera, &GlobalTransform), With<UserCamera>>,
        selected:       Query<&Transform, With<Selected>>,
        context:        Query<Entity, With<ContextMenu>>,
) {
    if context.is_empty() {
        return;
    }

    if let Ok(camera) = camera.get_single() {
        if let Ok(select_context) = selected.get_single() {
            if let Some(pos) = camera.0.world_to_viewport(camera.1, select_context.translation) {
                let window = window.single();
                context_res.procent_pos = Vec2::new(
                    (pos.x / window.resolution.physical_width() as f32) * 100.0, 
                    (pos.y / window.resolution.physical_height() as f32) * 100.0
                );
            }
        }
    }
}

fn def_context_menu(
    mut commands:   Commands,
        query:      Query<Entity, With<Selected>>,
        context:    Query<Entity, With<ContextMenu>>,
        key_input:  Res<ButtonInput<KeyCode>>
) {
    if query.is_empty() && context.is_empty() {
        return;
    }

    if query.is_empty() && !context.is_empty() {
        if let Ok(entity) = context.get_single() {
            commands.entity(entity).despawn_recursive();
        }
    }
    
    if !query.is_empty() && context.is_empty() {
        if key_input.just_pressed(KeyCode::AltLeft) {
            build_context_menu(&mut commands);
        }
    }
}

fn track_context(
    mut context:        Query<&mut Style, With<ContextMenu>>,
        context_res:    ResMut<ContextUIPos>,
) {
    if context.is_empty() {
        return;
    }

    if let Ok(mut context) = context.get_single_mut() {
        context.left    = Val::Percent(context_res.procent_pos.x + 3.0);
        context.top     = Val::Percent(context_res.procent_pos.y - 3.0);
    }
}

// ==============================
//  Context Menu
//  Buttons
// ==============================

#[derive(Component)]
struct AboutButton;

fn interact_with_about_context_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AboutButton>),
    >,
) {
    if button_query.is_empty() {
        return;
    }

    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            
            Interaction::Pressed => {
                *background_color = Color::rgb(0.20, 0.20, 0.20).into();
            }

            Interaction::Hovered => {
                *background_color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            
            Interaction::None => {
                *background_color = Color::rgb(0.22, 0.22, 0.22).into();
            }
        }
    }
}

#[derive(Component)]
struct TakeButton;

fn interact_with_take_context_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<TakeButton>),
    >,
    mut event: EventWriter<TakeItem>,
        select:Query<Entity, With<Selected>>,
        user:  Query<Entity, With<UserControl>>,
) {
    if button_query.is_empty() {
        return;
    }

    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {

            Interaction::Pressed => {
                if !user.is_empty() {
                    *background_color = Color::rgb(0.19, 0.19, 0.19).into();

                    let entity = user.single();
                    let selected = select.single();
                    
                    event.send(TakeItem(entity, Some(selected)));
                }
            }

            Interaction::Hovered => {
                *background_color = Color::rgb(0.24, 0.24, 0.24).into();
            }

            Interaction::None => {
                if user.is_empty() {
                    *background_color = Color::rgb(0.17, 0.17, 0.17).into();
                } else {
                    *background_color = Color::rgb(0.21, 0.21, 0.21).into();
                }
            }
        }
    }
}