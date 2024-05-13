#[allow(unused)]
use bevy::prelude::*;

use serde::Deserialize;

use bevy_inspector_egui::InspectorOptions;

/// Всё, что может бить предметом
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component, Reflect)]
pub enum ItemType {
    None,
    Tool(Tool),
    Weapon(Weapon),
    Item(Item),
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::None
    }
}

/// Всё, что может быть взято как иструмент
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component, Reflect)]
pub enum Tool {
    Axe,
    Shovel,
    Hoe,
    Pickaxe,
}

/// Всё, что может быть взято как оружие
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component, Reflect)]
pub enum Weapon {
    Gun,
}

/// Всё, что является предметом
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component, Reflect)]
pub enum Item {
    Material(Material),
    Consumables,
    Ammo,
}

/// Перечисление того, что является предметом
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component, Reflect)]
pub enum Material {
    Wood,
    Cobblestone,
    Flint,
}

/// Перечисление того, что является боеприпасом
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component, Reflect)]
pub enum Ammo {
    SmallAmmo,
    MediumAmmo,
    LargeAmmo,
    Rocket,
}

/// Структура для того, что может быть поднято
#[allow(unused)]
#[derive(Component, InspectorOptions)]
pub struct Pickupable {
    pub(crate) item: ItemType,
    pub count: usize
}

#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component, Reflect)]
pub struct ItemAndCount {
    pub item: ItemType,
    pub count: usize,
}

impl std::fmt::Display for ItemAndCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x {:?}", self.count, self.item)
    }
}