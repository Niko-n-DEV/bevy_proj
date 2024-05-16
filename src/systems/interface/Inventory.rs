#![allow(unused)]
use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    Container::InventoryItemSlot
};

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