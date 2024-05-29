use bevy::{
    prelude::*, 
    window::PrimaryWindow
};

use crate::core::{
    AppState,
    Camera::UserCamera,
    interface::game_ui::Select::Selected
};

pub fn context_menu_plugin(app: &mut App) {
    // Init Res
    app.init_resource::<ContextMenuRes>(); 
    app.add_systems(PreUpdate, track_pos_context.run_if(in_state(AppState::Game)));
    app.add_systems(Update, 
        (
                interact_with_context_buttons,
                track_context.after(track_pos_context)
            ).run_if(in_state(AppState::Game))
        );
    app.add_systems(PostUpdate, def_context_menu.run_if(in_state(AppState::Game)));
}

#[derive(Component)]
struct ContextMenu;

#[derive(Resource, Default)]
struct ContextMenuRes {
    pub procent_pos: Vec2
}

fn track_pos_context(
    mut context_res:    ResMut<ContextMenuRes>,
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
        query:      Query<&Transform, With<Selected>>,
        context:    Query<Entity, With<ContextMenu>>,
) {
    if query.is_empty() && !context.is_empty() {
        if let Ok(entity) = context.get_single() {
            commands.entity(entity).despawn_recursive();
        }
    }
    
    if !query.is_empty() && context.is_empty() {
        commands.spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(25.0),
                    width:  Val::Percent(15.0),
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
            ContextMenu
        )).with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        height: Val::Percent(25.),
                        width:  Val::Percent(100.),
                        ..default()
                    },
                    background_color:   Color::rgb(0.47, 0.47, 0.47).into(),
                    ..default()
                },
                AboutButton
            ));
        });
    }
}

fn track_context(
    mut context:        Query<&mut Style, With<ContextMenu>>,
        context_res:    ResMut<ContextMenuRes>,
) {
    if context.is_empty() {
        return;
    }

    if let Ok(mut context) = context.get_single_mut() {
        context.left    = Val::Percent(context_res.procent_pos.x + 3.0);
        context.top     = Val::Percent(context_res.procent_pos.y - 3.0);
    }
}

#[derive(Component)]
struct AboutButton;

fn interact_with_context_buttons(
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
                *background_color = Color::rgb(0.40, 0.40, 0.40).into();
            }
            Interaction::Hovered => {
                *background_color = Color::rgb(0.50, 0.50, 0.50).into();
            }
            Interaction::None => {
                *background_color = Color::rgb(0.47, 0.47, 0.47).into();
            }
        }
    }
}