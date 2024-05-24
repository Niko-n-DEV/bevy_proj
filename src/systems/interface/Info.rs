use bevy::prelude::*;

use bevy_egui::{
    egui,
    EguiContexts
};

use crate::core::{
    UserSystem::{
        Selected,
        // CursorPosition,
        CursorProcentPos
    },
    Item::EntityItem,
    ItemType::ItemEntity,
    // Object::EntityObject,
    resource::{
        graphic::Atlas::AtlasRes,
        Registry::Registry
    },
    ContainerSystem::CursorContainer
};

pub fn info_item_panel(
        info_query: Query<(&EntityItem, &ItemEntity), With<Selected>>,
    mut contexts:   EguiContexts,
) {
    if info_query.is_empty() {
        return;
    }

    if let Ok(info) = info_query.get_single() {
        egui::Window::new("Info")
            // .current_pos(Pos2::new(500.0, 500.0))
            .show(contexts.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("ID: {}", info.0.id_name));
                        ui.label(format!("Name: {}", info.0.name));
                        ui.label(format!("Health: {:?}", info.0.health));
                        ui.label(format!("Pos: {:?}", info.0.position));
                    });

                    ui.vertical(|ui| {
                        ui.label(format!("item: {:?}", info.1.item));
                        ui.label(format!("count: {}", info.1.count));
                    })
                })
            });
    }
}

// ==============================
// Preview Grab Cursor
// ==============================

#[derive(Component)]
pub struct CursorPreview;

pub fn cursor_grab(
    mut commands:   Commands,
        preview:    Query<Entity, With<CursorPreview>>,
        cursor_p:   Res<CursorProcentPos>,
        cursor_inv: Res<CursorContainer>,
    //    cursor:     Res<CursorPosition>,
        register:   Res<Registry>,
        atlas:      Res<AtlasRes>,
) {
    if cursor_inv.is_changed() {
        if cursor_inv.slot.is_none() && preview.is_empty() {
            return;
        }

        if cursor_inv.slot.is_none() && !preview.is_empty() {
            for entity in &preview {
                commands.entity(entity).despawn_recursive();
            }
            return;
        }

        if !cursor_inv.slot.is_none() && preview.is_empty() {
            if let Some(slot) = &cursor_inv.slot {
                if let Some(info) = register.get_item_info(&slot.name) {
                    if let Some(img) = atlas.items.extruct_texture(&info.id_texture) {
                        commands.spawn((
                            ImageBundle {
                                style: Style {
                                    position_type:  PositionType::Absolute,
                                    left:           Val::Percent(cursor_p.0.x + 10.0),
                                    top:            Val::Percent(cursor_p.0.y - 1.0),
                                    height:         Val::Percent(5.0),
                                    width:          Val::Percent(3.0),
                                    ..default()
                                },
                                image: UiImage::new(img.1.clone()),
                                ..default()
                            },
                            // SpriteBundle {
                            //     texture: img.1,
                            //     transform: Transform {
                            //         translation: Vec3::new(cursor.0.x + 2.0, cursor.0.y - 2.0, 100.),
                            //         scale: Vec3::splat(0.25),
                            //         ..default()
                            //     },
                            //     ..default()
                            // },
                            img.0,
                            CursorPreview
                        )).with_children(|parent| {
                            parent.spawn(
                                TextBundle {
                                    style: Style {
                                        position_type:  PositionType::Absolute,
                                        left:           Val::Percent(1.0),
                                        top:            Val::Percent(-1.0),
                                        ..default()
                                    },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            format!("{}", slot.count),
                                            TextStyle {
                                                font_size: 8.0,
                                                ..default()
                                            },
                                        )],
                                        ..default()
                                    },
                                    ..default()
                                }
                                // Text2dBundle {
                                //     text: Text {
                                //         sections: vec![TextSection::new(
                                //             format!("{}", slot.count),
                                //             TextStyle {
                                //                 font_size: 12.0,
                                //                 ..default()
                                //             },
                                //         )],
                                //         ..default()
                                //     },
                                //     transform: Transform {
                                //         translation: Vec3::new(4.0, -4.0, 1.5),
                                //         scale: Vec3::splat(0.75),
                                //         ..default()
                                //     },
                                //     ..default()
                                // }
                            );
                        });
                    }
                }
            }
        }
    }
}

/// Создаёт спрайт указанного предмета, который следует за курсором
/// 
/// Используется для перетаскивания и показа выбранного элемента
pub fn hover_item(
    mut hover:      Query<&mut Style, With<CursorPreview>>,
        cursor_p:   Res<CursorProcentPos>,
) {
    if hover.is_empty() {
        return;
    }

    if let Ok(mut hover_style) = hover.get_single_mut() {
        //hover_transform.translation = Vec3::new(cursor.0.x + 2.0, cursor.0.y - 2.0, 1.5)
        hover_style.left    = Val::Percent(cursor_p.0.x + 1.0);
        hover_style.top     = Val::Percent(cursor_p.0.y - 1.0);
    }
}
