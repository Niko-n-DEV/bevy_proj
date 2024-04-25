// #![allow(unused)]
use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    AppState,
    // Debug::{GameError, GameErrorType},
    items::ItemType::{ItemAndCount, ItemType},
};


pub const INVENTORY_SIZE: usize = 3 * 3;
pub const INVENTORY_ITEM_SIZE: usize = 16;

#[derive(Component, Default, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Container {
    //pub items: Vec<Option<InventoryItemSlot>>,
    pub slots: [InventoryItemSlot; 6]
}

#[derive(Component, Default, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct InventoryItemSlot {
    pub item_stack: ItemStack,
    pub slot: usize,
}

#[derive(Component, Default, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct ItemStack {
    pub item_type: ItemType,
    pub count: usize,
}

// // impl Default for Container {
// //     fn default() -> Self {
// //         Self {
// //             size: None
// //         }
// //     }
// // }

// #[derive(Component, InspectorOptions, Clone)]
// pub struct Inventory {
//     pub items: Container,
//     pub equipment_items: Container,
//     pub accessory_items: Container,
//     pub crafting_items: Container,
//     // pub crafting_result_item: Container,
// }

// #[derive(Component, Debug, PartialEq, Reflect, Default, Clone)]
// //#[reflect(Schematic, Default)]
// pub struct ItemStack {
//     // pub obj_type: WorldObject,
//     pub count: usize,
//     // pub rarity: ItemRarity,
//     // pub attributes: ItemAttributes,
//     // pub metadata: ItemDisplayMetaData,
// }

// #[derive(Component, Debug, PartialEq, Clone)]
// pub struct InventoryItemStack {
//     pub item_stack: ItemStack,
//     pub slot: usize,
// }

// impl InventoryItemStack {
//     pub fn new(item_stack: ItemStack, slot: usize) -> Self {
//         Self { item_stack, slot }
//     }

    
//     // pub fn drop_item_on_slot(
//     //     &self,
//     //     container: &mut Container,
//     //     inv_slots: &mut Query<&mut InventorySlotState>,
//     //     slot_type: InventorySlotType,
//     // ) -> Option<ItemStack> {
//     //     let obj_type = self.item_stack.obj_type;
//     //     let target_item_option = container.items[self.slot].clone();
//     //     if let Some(target_item) = target_item_option {
//     //         if target_item.get_obj() == &obj_type
//     //             && target_item.item_stack.metadata == self.item_stack.metadata
//     //             && target_item.item_stack.attributes == self.item_stack.attributes
//     //             && !(slot_type.is_equipment() || slot_type.is_accessory())
//     //         {
//     //             mark_slot_dirty(self.slot, slot_type, inv_slots);
//     //             return container.merge_item_stacks(self.item_stack.clone(), target_item);
//     //         } else {
//     //             return Some(container.swap_items(
//     //                 self.item_stack.clone(),
//     //                 self.slot,
//     //                 inv_slots,
//     //                 slot_type,
//     //             ));
//     //         }
//     //     } else if self
//     //         .item_stack
//     //         .clone()
//     //         .try_add_to_target_inventory_slot(self.slot, container, inv_slots)
//     //         .is_err()
//     //     {
//     //         panic!("Failed to drop item on stot");
//     //     }

//     //     None
//     // }

// }

// pub const INVENTORY_SIZE: usize = 3 * 3;
// pub const INVENTORY_ITEM_SIZE: usize = 16;

// #[derive(Component, InspectorOptions, Clone)]
// pub struct Inventory {
//     pub items: [ItemAndCount; INVENTORY_SIZE],
// }

// pub struct InventoryOverflow(pub usize);

// impl Inventory {
//     pub fn add(&mut self, item_and_count: &ItemAndCount) -> Option<InventoryOverflow> {
//         let mut remaining_amount = item_and_count.count;

//         for item in self
//             .items
//             .iter_mut()
//             .filter(|item| item.item != ItemType::None)
//         {
//             if item.item == item_and_count.item {
//                 let addable_item_count =
//                     std::cmp::min(remaining_amount, INVENTORY_ITEM_SIZE - item_and_count.count);
//                 item.count += addable_item_count;
//                 remaining_amount -= addable_item_count;
//                 if remaining_amount == 0 {
//                     return None;
//                 }
//             }
//         }

//         for item in self
//             .items
//             .iter_mut()
//             .filter(|item| item.item == ItemType::None)
//         {
//             item.item = item_and_count.item;
//             let addable_item_count =
//                 std::cmp::min(remaining_amount, INVENTORY_ITEM_SIZE - item_and_count.count);
//             item.count = addable_item_count;
//             remaining_amount -= item.count;
//             if remaining_amount == 0 {
//                 return None;
//             }
//         }
//         Some(InventoryOverflow(remaining_amount))
//     }

//     pub fn can_add(&self, item_and_count: &ItemAndCount) -> bool {
//         let mut inventory_clone = self.clone();
//         inventory_clone.add(item_and_count).is_none()
//     }

//     pub fn remove(&mut self, item_and_count: &ItemAndCount) -> Result<(), GameError> {
//         let mut existing = false;
//         for inventory_item in self.items.iter_mut() {
//             if inventory_item.item == item_and_count.item {
//                 existing = true;
//                 if inventory_item.count > item_and_count.count {
//                     inventory_item.count -= item_and_count.count;
//                     return Ok(());
//                 }
//                 if inventory_item.count == item_and_count.count {
//                     inventory_item.count = 0;
//                     inventory_item.item = ItemType::None;
//                     return Ok(());
//                 }
//             }
//         }
//         if existing {
//             return Err(GameError::new(
//                 GameErrorType::ItemMissing,
//                 format!("Not enough items in inventory: {:?}", item_and_count.item),
//             ));
//         }
//         Err(GameError::new(
//             GameErrorType::ItemMissing,
//             format!("Item not in inventory: {:?}", item_and_count.item),
//         ))
//     }

//     pub fn can_remove(&self, item_and_count: &ItemAndCount) -> bool {
//         let mut inventory_clone = self.clone();
//         matches!(inventory_clone.remove(item_and_count), Ok(()))
//     }
// }

// pub struct InventoryPlugin;

// impl Plugin for InventoryPlugin {
//     fn build(&self, app: &mut App) {
//         // app
//         //     // .add_system(Update)
//         //     // .register_inspectable::<Inventory>();
//         // ;
//     }
// }
