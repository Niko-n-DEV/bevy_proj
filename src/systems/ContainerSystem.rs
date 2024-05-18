#![allow(unused)]
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Index, IndexMut},
    marker::PhantomData
};

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    AppState,
    ItemType::
        ItemType
    ,
    interface::{
        Inventory::{
            InventoryDisplayToggleEvent,
            toggle_inventory_open_event_send,
            toggle_inventory_open,
            inventory_update
        }
    }
};

pub struct ContainerPlugin<I: ItemTypeEx> {
    pub phantom: PhantomData<I>
}

impl<I: ItemTypeEx> Plugin for ContainerPlugin<I> {
    fn build(&self, app: &mut App) {
        app
            // Reg Type
            .register_type::<Inventory>()
            // Reg Events
            .add_event::<InventoryDisplayToggleEvent>()
            .add_event::<ItemPickUpEvent>()
            .add_event::<ItemDropEvent>()
            // Systems
            .add_systems(Update, 
                (
                //    BarGui::spawn_inventory_ui::<I>,
                    toggle_inventory_open_event_send::<I>,
                    toggle_inventory_open::<I>,
                ).run_if(in_state(AppState::Game))
            )
            .add_systems( Update,
                (
                    inventory_update::<I>
                ).run_if(in_state(AppState::Game))
            )
        ;
    }
}

// ==============================
// Event
// ==============================


#[derive(Event, Debug, Copy, Clone)]
pub struct ItemPickUpEvent {
    pub picker: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
pub struct ItemDropEvent {
    pub droper: Entity,
    pub item: Entity,
}

// ==============================
// Container
// ==============================

pub const INVENTORY_SIZE: usize = 3 * 3;
pub const INVENTORY_ITEM_SIZE: usize = 16;

#[derive(Component, Default, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Container {
    pub slots: [InventoryItemSlot; 6]
}

impl Container {
    pub fn find_in_container(
        &self, 
        item_type: ItemType
    ) -> Option<&InventoryItemSlot> {
        for slot in &self.slots {
            if slot.item_stack.item_type == item_type {
                return Some(slot);
            }
        }
        None
    }

    /// Взять весь слот
    pub fn take_from_container(
        &mut self, 
        item_type: ItemType
    ) -> Option<&mut InventoryItemSlot> {
        for slot in &mut self.slots {
            if slot.item_stack.item_type == item_type {
                return Some(slot);
            }
        }
        None
    }

    /// Добавляет в инвентарь предмет в первый же свободный слот.
    pub fn add_in_container(
        &mut self,
            item: ItemType,
            count: usize
    ) -> bool {

        for slot in &mut self.slots {
            // Добавление в имеющийся стак, если подбираемый предмет уже есть
            if slot.item_stack.item_type == item {
                slot.item_stack.count += count;

                return true;
            }
            // Добавление в пустой слот
            if slot.item_stack.item_type == ItemType::None {
                slot.item_stack.item_type = item;
                slot.item_stack.count = count;
                
                return true;
            }
        }
        
        false
    }

    /// Добавляет в инвентарь предмет в определённый слот
    pub fn place_in_container(
        &mut self,
            item: ItemType,
            count: usize,
            slot: usize
    ) {
        if self.slots[slot].item_stack.item_type == ItemType::None {
            self.slots[slot].item_stack.item_type = item;
            self.slots[slot].item_stack.count = count;
        } else {
            println!("В слоте уже что-то есть")
        }
    }

    /// Взять определённое кол-во из слота
    pub fn take_def_from_container(
        &mut self,
        item_type: ItemType,
        count: usize
    ) -> Option<ItemStack> {
        if let Some(slot) = self.take_from_container(item_type) {
            if slot.item_stack.count >= count {
                slot.item_stack.count -= count;
                return Some(ItemStack {
                    item_type: slot.item_stack.item_type,
                    count,
                });
            }
            let taken_count = slot.item_stack.count;
            slot.item_stack.count = 0;
            return Some(ItemStack {
                    item_type: slot.item_stack.item_type,
                    count: taken_count,
                });
        }
        None
    }
}

#[derive(Component, Default, InspectorOptions, Clone, Copy, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct InventoryItemSlot {
    pub item_stack: ItemStack,
    pub slot: usize,
}

#[derive(Component, Default, InspectorOptions, Clone, Copy, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct ItemStack {
    //pub id_name: String,
    pub item_type: ItemType,
    pub count: usize,
}

// ==============================
// Equipment
// ==============================

pub trait ItemTypeEx: Component + Copy + Clone + Eq + Hash + Debug + Default {}

#[derive(Debug, Default, Clone, Component)]
pub struct Equipment<I: ItemTypeEx> {
    pub items: HashMap<(I, u8), Option<Entity>>,
}

// (З/А) - Заметка автора кода (не я автор)
impl<I: ItemTypeEx> Equipment<I> {

    /// Получение списка элементов
    pub fn list<T, V>(&self, t_items: &Query<&T, (With<I>, Without<V>)>) -> Vec<T>
    where
        T: Component + Clone,
        V: Component,
    {
        self.iter_some()
            .filter_map(|(_, e)| {
                if let Ok(t) = t_items.get(e) {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Добавление в свободный слот
    pub fn add(&mut self, item: Entity, item_type: &I) -> bool {
        if let Some((_, item_slot)) = self
            .items
            .iter_mut()
            .find(|((t, _), b)| t == item_type && b.is_none())
        {
            *item_slot = Some(item);
            true
        } else {
            false
        }
    }
    // TODO: Реализация замены добавляемого предмета и возврат существующего (З/А)

    /// Изъятие из слота определённого придмета
    pub fn take(&mut self, item: Entity) -> bool {
        if let Some((_, e)) = self
            .items
            .iter_mut()
            .find(|(_, b)| b.is_some() && b.unwrap() == item)
        {
            *e = None;
            true
        } else {
            false
        }
    }

    /// Что-то
    pub fn iter_some(&'_ self) -> impl Iterator<Item = ((I, u8), Entity)> + '_ {
        // TODO: вместо этого используйте filter_map (З/А)
        self.items
            .iter()
            .filter(|(_, i)| i.is_some())
            .map(move |(a, i)| (*a, i.unwrap()))
    }
}

impl<I: ItemTypeEx> Index<(I, u8)> for Equipment<I> {
    type Output = Option<Entity>;
    
    /// Получения предмета по индексу
    fn index(&self, index: (I, u8)) -> &Self::Output {
        if let Some(item) = self.items.get(&index) {
            return item;
        }
        &None
    }
}

impl<I: ItemTypeEx> IndexMut<(I, u8)> for Equipment<I> {
    fn index_mut(&mut self, index: (I, u8)) -> &mut Self::Output {
        if let Some(ee) = self.items.get_mut(&index) {
            return ee;
        }
        panic!("No item with index {:?}", index);
    }
}

// ==============================
// Inventory
// ==============================

#[derive(Debug, Clone, Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Inventory {
    items: Vec<Option<(Entity, String, usize)>>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::with_capacity(Inventory::DEFAULT_CAPACITY)
    }
}

impl Inventory {
    pub const DEFAULT_CAPACITY: usize = 12;

    // Установка размера инвентаря по умолчанию
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: vec![None; cap],
        }
    }

    /// Добавление в первый попавшиеся свободный слот
    // pub fn add(&mut self, item: (Entity, usize)) -> bool {
    //     if let Some((_, e)) = self.items.iter_mut().enumerate().find(|(_, b)| b.is_none()) {
    //         *e = Some((item.0, item.1));
    //         true
    //     } else {
    //         false
    //     }
    // }
    pub fn add(&mut self, item: (Entity, String, usize)) -> bool {
        // Проверяем, есть ли слот с таким же именем
        if let Some((_, e)) = self.items.iter_mut().enumerate().find(|(_, b)| {
            if let Some((entity, name, count)) = b {
                name == &item.1
            } else {
                false
            }
        }) {
            if let Some((entity, name, count)) = e {
                *count += item.2;
                true
            } else {
                false
            }
        } else {
            // Если слот с таким именем не найден, ищем свободный слот
            if let Some((_, e)) = self.items.iter_mut().enumerate().find(|(_, b)| b.is_none()) {
                *e = Some((item.0, item.1.clone(), item.2));
                true
            } else {
                false
            }
        }
    }

    /// Взятие предмета из инвентаря
    // pub fn take(&mut self, item: (Entity, String, usize)) -> bool {
    //     if let Some((_, e)) = self
    //         .items
    //         .iter_mut()
    //         .enumerate()
    //         .find(|(_, b)| b.is_some() && b.unwrap() == item)
    //     {
    //         *e = None;
    //         true
    //     } else {
    //         false
    //     }
    // }
    pub fn take(&mut self, item: (Entity, String, usize)) -> bool {
        if let Some(e) = self.items.iter_mut().find(|b| {
            if let Some((entity, name, count)) = b {
                entity == &item.0 && name == &item.1 && *count >= item.2
            } else {
                false
            }
        }) {
            if let Some((_, _, count)) = e {
                if *count > item.2 {
                    *count -= item.2;
                } else {
                    *e = None;
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    
    /// Проверка всех слотов и возвращение Предмета
    // pub fn iter_some(&self) -> impl Iterator<Item = (Entity, &str, usize)> + '_ {
    //     self.items.iter().filter_map(|i| *i)
    // }
    pub fn iter_some(&self) -> impl Iterator<Item = (Entity, &str, usize)> + '_ {
        self.items.iter().filter_map(|i| {
            if let Some((entity, name, count)) = i {
                Some((*entity, name.as_str(), *count))
            } else {
                None
            }
        })
    }

    /// Проверка, полон ли инвентарь
    pub fn is_full(&self) -> bool {
        self.items.iter().all(|i| i.is_some())
    }

    /// Размер занимаемого пространства
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Проверка, пуст ли инвентарь
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Index<usize> for Inventory {
    type Output = Option<(Entity, String, usize)>;

    /// Возвращает элемент по указанному индексу
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}