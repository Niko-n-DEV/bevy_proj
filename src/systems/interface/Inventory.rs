#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    Container::InventoryItemSlot
};

#[derive(Component)]
pub struct InventoryGui {
    pub slots: [InventorySlot; 12]
}

#[derive(Component, Default, Clone, Copy)]
pub struct InventorySlot {
    pub id: usize,
    pub contain: Option<InventoryItemSlot>
}

impl InventorySlot {
    pub fn new(id: usize) -> Self {
        Self { 
            id,
            contain: None
        }
    }
}