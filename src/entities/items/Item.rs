use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, InspectorOptions)]
pub struct Pickupable {
    pub item: ItemType,
}

/// Всё, что может бить предметом 
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ItemType {
    None,
    Tool(Tool),
    Weapon,
    Item,
}

/// Всё, что может быть взято как иструмент
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Tool {
    Axe,
    Shovel,
    Hoe,
    Pickaxe
}

#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Item {
    Material,
    Consumables,
    Ammo
}

#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Ammo {
    SmallAmmo,
    MediumAmmo,
    LargeAmmo,
    Rocket
}