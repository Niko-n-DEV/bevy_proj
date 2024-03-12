#[derive(Component, InspectorOptions)]
pub struct Pickupable {
    pub(crate) item: ItemType,
}

/// Всё, что может бить предметом 
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
pub enum ItemType {
    None,
    Tool(Tool),
    Weapon,
    Material
}

/// Всё, что может быть установлено
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
pub enum Tool {
    Axe,
    Shovel,
    Hoe,
    Pickaxe
}