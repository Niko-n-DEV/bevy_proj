use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    ItemType::ItemType,
    ContainerSystem::{
        Slot,
        Inventory,
        ItemTypeEx,
        CursorContainer,
        InventoryItemSlot,
    },
    UserSystem::UserControl,
    resource::{
        graphic::Atlas::{
            AtlasType,
            AtlasRes,
        },
        Registry::Registry
    },
};

use super::{BarGui::BarGui, GameUI};

#[allow(unused)]
// Нужно решить, как правльно использовать это в проекте
pub fn inventory_plugin(app: &mut App) {

}

/// Компонент отвечающий ща GUI инвентаря игрока
#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct InventoryGui {
    pub slots: [InventorySlot; 12]
}

impl InventoryGui {
    pub fn get_slot(&self, slot_id: usize) -> Option<InventorySlot> {
        if slot_id < self.slots.len() {
            Some(self.slots[slot_id].clone())
        } else {
            None
        }
    }
}

#[derive(Component, Clone, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct InventorySlot {
    pub id:         Option<usize>,
    pub entity:     Option<Entity>,
    pub contain:    Option<InventoryItemSlot>
}

impl InventorySlot {
    pub fn new(id: Option<usize>, entity: Option<Entity>) -> Self {
        Self { 
            id,
            entity,
            contain: None
        }
    }
}

impl Default for InventorySlot {
    fn default() -> Self {
        Self {
            id:         None,
            entity:     None,
            contain:    None
        }
    }
}

// ==============================
// Inventory User
// ==============================
    
pub fn toggle_inventory_open_event_send<I: ItemTypeEx>(
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Query<
        Entity,
        (
            With<UserControl>,
            With<Inventory>,
        ),
    >,
    mut inventory_toggle_writer: EventWriter<InventoryDisplayToggleEvent>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        if let Ok(player) = player.get_single() {
            inventory_toggle_writer.send(InventoryDisplayToggleEvent { actor: player });
        }
    }
}

pub(crate) fn toggle_inventory_open<I: ItemTypeEx>(
    mut cmd:        Commands,
    mut inv_toggle: EventReader<InventoryDisplayToggleEvent>,
    mut bar_gui:    Query<(Entity, &mut BarGui), With<BarGui>>,
        game_ui:    Query<&GameUI, With<GameUI>>,
        user:       Query<&Inventory>,
        inv_displ:  Query<(Entity, &InventoryDisplayOwner)>,
) {
    // if bar_gui.is_empty() || game_ui.is_empty() || user.is_empty() {
    //     return;
    // }

    if inv_toggle.is_empty() {
        return;
    }
    
    for e in inv_toggle.read() {

        // Inventory Spawn
        if let Ok(game_ui) = game_ui.get_single() {
            if game_ui.bargui_is_open {
                if let Ok(mut bar_gui) = bar_gui.get_single_mut() {

                    let inventory = if let Ok(player) = user.get(e.actor) {
                        player
                    } else {
                        bevy::log::error!("InventoryDisplayToggleEvent with invalif actor_id (missing EquipmentDisplay, Inventory)");
                        return;
                    };
            
                    if let Some((inventory_display_entity, _)) =
                        inv_displ.iter().find(|(_, o)| o.actor == e.actor)
                    {
                        cmd.entity(inventory_display_entity).despawn_recursive();
                        bar_gui.1.inventory_open = !bar_gui.1.inventory_open;
                        
                        return;
                    }

                    if !bar_gui.1.inventory_open {
                        cmd.entity(bar_gui.0).with_children(|parent| {
                            parent.spawn((
                                Name::new("Inventory"),
                                InventoryDisplayOwner   { actor: e.actor },
                                InventoryDisplayNode    { id: e.actor },
                                NodeBundle {
                                    style: Style {
                                        display:    Display::Grid,
                                        left:       Val::Px(-179.0),
                                        bottom:     Val::Px(3.0),
                                        width:      Val::Px(136.0),
                                        height:     Val::Px(104.0),
                                        border:     UiRect::all(Val::Px(4.0)),
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
                                Inventory::with_capacity(12)
                            )).with_children(|slots| {
                                for index in 0..inventory.len() {
                                    slots.spawn((
                                        Name::new(format!("Slot {index}")),
                                        InventoryDisplaySlot { index, slot: None },
                                        Interaction::default(),
                                        NodeBundle {
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
                                        }
                                    ));
                                }
                            }).with_children(|parent| {
                                parent.spawn(
                                    NodeBundle {
                                        style: Style {
                                            position_type:  PositionType::Absolute,
                                            left:           Val::Px(-4.0),
                                            bottom:         Val::Px(100.0),
                                            width:          Val::Px(136.0),
                                            height:         Val::Px(20.0),
                                            ..default()
                                        },
                                        background_color:   Color::rgb(0.19, 0.19, 0.19).into(),
                                        ..default()
                                    }
                                );
                            });
                        });

                        bar_gui.1.inventory_open = !bar_gui.1.inventory_open;
                    }
                }
            }
        }                    
    }
}

pub(crate) fn inventory_update<I: ItemTypeEx>(
    mut cmd:                Commands,
    mut inv_slots:          Query<&mut InventoryDisplaySlot>,
        register:           Res<Registry>,
        atlas:              Res<AtlasRes>,
        game_ui:            Query<&GameUI, With<GameUI>>,
        inv_displ_nodes:    Query<(&InventoryDisplayNode, &Children)>,
        player_inv:         Query<&Inventory, With<UserControl>>,
) {
    if game_ui.is_empty() {
        return;
    }
    
    // Прогон по узлу со слотами и их дочерними элементами
    for (display_node, display_node_children) in inv_displ_nodes.iter() {
        let inventory = if let Ok(inventory) = player_inv.get(display_node.id) {
            inventory
        } else {
            bevy::log::error!("InventoryDisplayNode without associated Inventory");
            continue;
        };

        for &slot_entity in display_node_children.iter() {
            let mut slot = if let Ok(slot) = inv_slots.get_mut(slot_entity) {
                slot
            } else {
                // bevy::log::error!(
                //     "InventoryDisplayNode's child is not a InventoryDisplaySlot. Should be."
                // );
                continue;
            };
            
            let mut slot_cmd = cmd.entity(slot_entity);

            if let Some(item_entity) = &inventory[slot.index] {
                let render = if let Some(slot_item) = slot.slot.clone() {
                    if item_entity.name != slot_item.name || (item_entity.name == slot_item.name && slot_item.count != item_entity.count) {
                        slot.slot = Some(item_entity.clone());
                        
                        slot_cmd.despawn_descendants();
                        true
                    } else {
                        false
                    }
                } else {
                    slot.slot = Some(item_entity.clone());

                    true
                };

                if render {
                    if let Some(info) = register.get_item_info(&item_entity.id_name) {

                        match info.item_type {
                            
                            ItemType::Item(_) | ItemType::None => {
                                if let Some(img) = atlas.get_texture(AtlasType::Items, &info.id_texture) {
                                    slot_cmd.with_children(|cb| {
                                        cb.spawn((
                                            ImageBundle {
                                                image: UiImage::new(img.1),
                                                ..default()
                                            },
                                            img.0
                                        ));
                                        cb.spawn(TextBundle {
                                            style: Style {
                                                position_type:  PositionType::Absolute,
                                                top:            Val::Percent(55.0),
                                                left:            Val::Percent(70.0),
                                                ..default()
                                            },
                                            text: Text {
                                                sections: vec![TextSection::new(
                                                    format!("{}", item_entity.count),
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
                                } else {
                                    bevy::log::error!(
                                        "item in inventory but not in the world. Or missing UiRenderInfo."
                                    );
                                }
                            },
        
                            ItemType::Weapon(_) => {
                                if let Some(img) = atlas.get_texture(AtlasType::Weapon, &info.id_texture) {
                                    slot_cmd.with_children(|cb| {
                                        cb.spawn((
                                            ImageBundle {
                                                image: UiImage::new(img.1),
                                                ..default()
                                            },
                                            img.0
                                        ));
                                        cb.spawn(TextBundle {
                                            style: Style {
                                                position_type:  PositionType::Absolute,
                                                top:            Val::Percent(55.0),
                                                left:            Val::Percent(70.0),
                                                ..default()
                                            },
                                            text: Text {
                                                sections: vec![TextSection::new(
                                                    format!("{}", item_entity.count),
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
                                } else {
                                    bevy::log::error!(
                                        "Weapon item in inventory but not in the world. Or missing UiRenderInfo."
                                    );
                                }
                            },
        
                            ItemType::Tool(_) => {
                                
                            }
                        }
                    }
                }
            } else {
                slot.slot = None;
                slot_cmd.despawn_descendants();
            }
        }
    }
}

pub fn inventory_click_item(
    mut cursor_inv:     ResMut<CursorContainer>,
    mut player_inv:     Query<&mut Inventory, With<UserControl>>,
    mut interact_slots: Query<(&Interaction, &mut BackgroundColor, &InventoryDisplaySlot), Changed<Interaction>>,
) {
    for (interaction, mut color, slot) in &mut interact_slots {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(mut inv) = player_inv.get_single_mut() {
                    // Буферная переменная для временного хранения предмета из курсора
                    let mut buffer_item = None;

                    // Если в курсоре есть предмет
                    if let Some(cursor_item) = cursor_inv.slot.take() {
                        // Если в слоте есть предмет, помещаем его в курсор
                        if let Some(slot_item) = inv.get_slot_mut(slot.index).and_then(|slot_mut| slot_mut.take()) {
                            cursor_inv.slot = Some(slot_item);
                            // Помещаем предмет из курсора в буфер
                            buffer_item = Some(cursor_item);
                        } else {
                            // Если в слоте нет предмета, помещаем предмет из курсора в слот
                            inv.add_to_slot(slot.index, Some(cursor_item));
                        }
                    } 
                    // Если в курсоре нет предмета, но есть предмет в слоте, забираем его
                    else if let Some(slot_item) = inv.get_slot_mut(slot.index).and_then(|slot_mut| slot_mut.take()) {
                        cursor_inv.slot = Some(slot_item);
                    }

                    // Если буфер не пуст, помещаем его в слот
                    if let Some(buffer_item) = buffer_item {
                        inv.add_to_slot(slot.index, Some(buffer_item));
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.28, 0.28, 0.28).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.24, 0.24, 0.24).into();
            }
        }
    }
}

// ==============================
// Other Inventories
// ==============================

/// Компонент отвечающий за GUI инвентаря сторонних хранилищ
#[derive(Debug, Clone, Component)]
pub struct InventoryDisplayOwner {
    pub actor: Entity,
}

/// Узел, содержащий дочерние элементы InventoryDisplaySlot
#[derive(Debug, Clone, Component)]
pub struct InventoryDisplayNode {
    /// ID объекта субъекта, имеющего этот инвентарь
    pub id: Entity,
}

// Для инвентаря
// #[derive(Default, Debug, Clone, PartialEq, Eq, Component)]
#[derive(Default, Debug, Clone, Component)]
pub struct InventoryDisplaySlot {
    pub index:  usize,
    pub slot:   Option<Slot>,
}

// // Для инвентаря эквипа
// #[derive(Default, Debug, Clone, Component)]
// pub struct EquipmentDisplaySlot<I: ItemTypeEx> {
//     pub index: (I, u8),
//     pub item: Option<Entity>,
//     pub is_dummy_rendered: bool,
// }

#[derive(Event, Debug, Copy, Clone)]
pub struct InventoryDisplayToggleEvent {
    /// ID субъекта, желающего переключить отображение инвентаря
    pub actor: Entity,
}

// ==============================
// Item Type Interaction
// ==============================

// #[derive(Debug, Clone, Component)]
// pub struct Equipable {
//     actor: Entity,
//     item: Entity,
// }

// #[derive(Debug, Clone, Component)]
// pub struct Unequipable {
//     actor: Entity,
//     item: Entity,
// }

// ==============================
// UiRender Image
// Сомнительно, ну, окей
// ==============================

// /// Указывает, как отображать материал, если он размещен на дисплее инвентаря или оборудования
// #[derive(Default, Debug, Clone, Component)]
// pub struct UiRenderInfo {
//     pub image: UiImage,
// }

// pub trait ItemTypeUiImage<I: ItemTypeEx>: Resource {
//     fn get_image(&self, item_type: I) -> UiImage;
// }

// // Для хранения изображения слота, когда курсор наведён на слот и шрифт
// #[derive(Debug, Clone, Resource)]
// pub struct InventoryUiAssets {
//     pub slot:               Handle<Image>,  // Относительно
//     pub hover_cursor_image: Handle<Image>,  // Возможно
//     pub font:               Handle<Font>,
// }