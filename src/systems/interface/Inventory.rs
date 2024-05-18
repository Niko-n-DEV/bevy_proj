#![allow(unused)]
use bevy::{
    prelude::*,
};

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    ContainerSystem::{
        Inventory,
        Container,
        ItemTypeEx,
        InventoryItemSlot
    },
    interface::GameUI::{
        BarGui,
        GameUI
    },
    UserSystem::User,
    resource::Registry::Registry,
};

/// Компонент отвечающий ща GUI инвентаря игрока
#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct InventoryGui {
    pub slots: [InventorySlot; 12]
}

impl InventoryGui {
    pub fn get_slot(&self, slot_id: usize) -> Option<InventorySlot> {
        if slot_id < self.slots.len() {
            Some(self.slots[slot_id])
        } else {
            None
        }
    }
}

#[derive(Component, Clone, Copy, InspectorOptions, Reflect)]
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
            With<User>,
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
                                Interaction::default(),
                                NodeBundle {
                                    style: Style {
                                        display:    Display::Grid,
                                        left:       Val::Px(-176.0),
                                        bottom:     Val::Px(4.0),
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
                                Inventory::with_capacity(12)
                            )).with_children(|slots| {
                                let mut slot_entities = Vec::new();
                                
                                for index in 0..inventory.len() {
                                    let slot = slots.spawn((
                                        Name::new(format!("Slot {index}")),
                                        InventoryDisplaySlot { index, item: None },
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
                                    )).id();
                                    
                                    slot_entities.push(slot);
                                }
                                // event.send(InvSlotsBuild(slot_entities));
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
    mut bar_gui:            Query<(Entity, &mut BarGui), With<BarGui>>,
        register:           Res<Registry>,
        game_ui:            Query<&GameUI, With<GameUI>>,
    //    inv_options:        Res<InventoryDisplayOptions>,
        inv_displ_nodes:    Query<(&InventoryDisplayNode, &Children)>,
        invs:               Query<&Inventory>,
        items:              Query<&UiRenderInfo, With<I>>,
) {
    if game_ui.is_empty() && bar_gui.is_empty() {
        return;
    }
    
    for (display_node, display_node_children) in inv_displ_nodes.iter() {
        let inventory = if let Ok(inventory) = invs.get(display_node.id) {
            inventory
        } else {
            bevy::log::error!("InventoryDisplayNode without associated Inventory");
            continue;
        };

        for &slot_entity in display_node_children.iter() {
            let mut slot = if let Ok(slot) = inv_slots.get_mut(slot_entity) {
                slot
            } else {
                bevy::log::error!(
                    "InventoryDisplayNode's child is not a InventoryDisplaySlot. Should be."
                );
                continue;
            };

            let mut slot_cmd = cmd.entity(slot_entity);
            if let Some(item_entity) = &inventory[slot.index] {
                let render = if let Some(slot_item) = slot.item {
                    if item_entity.0 != slot_item {
                        slot.item = Some(item_entity.0);
                        slot_cmd.despawn_descendants();
                        true
                    } else {
                        false
                    }
                } else {
                    slot.item = Some(item_entity.0);
                    true
                };

                if render {
                    // if let Ok(info) = items.get(item_entity) {
                    //     slot_cmd.with_children(|cb| {
                    //         cb.spawn((
                    //             Interaction::default(),
                    //             //UiHoverTip::new(item_entity),
                    //             Equipable {
                    //                 actor: display_node.id,
                    //                 item: item_entity,
                    //             },
                    //             ImageBundle {
                    //                 image: info.image.clone(),
                    //                 ..Default::default()
                    //             },
                    //         ));
                    //     });
                    // } else {
                    //     bevy::log::error!(
                    //         "item in inventory but not in the world. Or missing UiRenderInfo."
                    //     );
                    // }
                }
            } else {
                slot.item = None;
                slot_cmd.despawn_descendants();
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Component)]
pub struct InventoryDisplaySlot {
    pub index: usize,
    pub item: Option<Entity>,
}

#[derive(Default, Debug, Clone, Component)]
pub struct EquipmentDisplaySlot<I: ItemTypeEx> {
    pub index: (I, u8),
    pub item: Option<Entity>,
    pub is_dummy_rendered: bool,
}

#[derive(Event, Debug, Copy, Clone)]
pub struct InventoryDisplayToggleEvent {
    /// ID субъекта, желающего переключить отображение инвентаря
    pub actor: Entity,
}

// Item

// TODO: move to bevy_inventory lib
#[derive(Debug, Clone, Component)]
pub struct Equipable {
    actor: Entity,
    item: Entity,
}
// TODO: move to bevy_inventory lib
#[derive(Debug, Clone, Component)]
pub struct Unequipable {
    actor: Entity,
    item: Entity,
}

// UiRender Image

/// Указывает, как отображать материал, если он размещен на дисплее инвентаря или оборудования
#[derive(Default, Debug, Clone, Component)]
pub struct UiRenderInfo {
    pub image: UiImage,
}

pub trait ItemTypeUiImage<I: ItemTypeEx>: Resource {
    fn get_image(&self, item_type: I) -> UiImage;
}

#[derive(Debug, Clone, Resource)]
pub struct InventoryUiAssets {
    pub slot:               Handle<Image>,
    pub hover_cursor_image: Handle<Image>,
    pub font:               Handle<Font>,
}