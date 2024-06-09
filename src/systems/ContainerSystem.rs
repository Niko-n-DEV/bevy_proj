// #![allow(unused)]
use bevy::prelude::*;

use std::{
    fmt::Debug,
    hash::Hash,
    ops::Index,
    marker::PhantomData
};

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{
    interface::game_ui::{
            Info::{
                cursor_grab,
                hover_item
            }, Inventory::{
                inventory_click_item, inventory_update, toggle_inventory_open, toggle_inventory_open_event_send, InventoryDisplayToggleEvent
            }
        },
    AppState,
    Item::ItemEntity,
    ItemType::{
        ItemStackType,
        ItemType
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
            // Init Resource
            .init_resource::<CursorContainer>()
            // Reg Events
            .add_event::<InventoryDisplayToggleEvent>()
            // .add_event::<ItemPickUpEvent>()
            // .add_event::<ItemDropEvent>()
            // Systems
            .add_systems(Update, 
                (
                //  BarGui::spawn_inventory_ui::<I>,
                    toggle_inventory_open_event_send::<I>,
                    toggle_inventory_open::<I>,
                    inventory_click_item,
                    hover_item
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(PostUpdate, cursor_grab.run_if(in_state(AppState::Game))
            )
            .add_systems(Update,
                (
                    inventory_update::<I>.after(toggle_inventory_open::<I>)
                ).run_if(in_state(AppState::Game))
            )
        ;
    }
}

#[derive(Component, Default, InspectorOptions, Clone, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct InventoryItemSlot {
    pub item_stack: ItemStack,
    pub slot: usize,
}

#[derive(Component, Default, InspectorOptions, Clone, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct ItemStack {
    // pub id_name: String,
    // pub item_type: ItemType,
    pub item_type:  ItemEntity,
    pub count:      usize,
}

// ==============================
// Equipment
// ==============================

#[allow(unused)]
#[derive(Debug, Default, Clone, Component)]
pub struct Equipment {
    pub head:   [Option<Entity>; 3], // Шлем | Очки | Маска
    pub body:   [Option<Entity>; 3], // Нагрудник | Майка | Штаны | Пояс | Рюкзак | Наплечная сумка    
    pub hands:  [Option<Entity>; 4], // Hands | ArmBand | Weapon left | Weapon right (Если оружие двуручное, то оно технически занимает два слота)
    pub legs:    Option<Entity>       // Ботинки   
}

// ==============================
// Cursor Contain
// ==============================

#[derive(Resource, Default)]
pub struct CursorContainer {
    pub slot: Option<Slot>
}

// ==============================
// Inventory
// ==============================

pub trait ItemTypeEx: Component + Copy + Clone + Eq + Hash + Debug + Default {}

#[derive(Debug, Clone, Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Inventory {
    items: Vec<Option<Slot>>,
}

#[derive(Debug, Clone, Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Slot {
    pub name:       String,
    pub id_name:    String,
    pub id_source:  Option<String>,
    pub item_type:  ItemType,
    pub durability: Option<usize>,
    pub stack_size: Option<usize>,
    pub stackable:  Option<ItemStackType>,
    pub count:      usize,
}

impl Slot {
    fn add_to_exist(&mut self, count: usize) -> usize {
        if self.check_stackable() {
            if let Some(stack_size) = self.stack_size {
                let remaining = (self.count + count).saturating_sub(stack_size);
                self.count = stack_size;
                remaining
            } else {
                self.count += count;
                0
            }
        } else {
            0
        }
    }

    pub fn check_stackable(&self) -> bool {
        if let Some(stack) = self.stackable {
            return stack.is_stackable()
        }
        false
    }

    pub fn is_full(&self) -> bool {
        self.stack_size.map_or(true, |stack_size| self.count == stack_size)
    }
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

    /// Добавление в первый попавшийся свободный слот
    pub fn add(&mut self, item: &mut ItemEntity) -> bool {
        if let Some(slot) = self.items.iter_mut().find(|slot| {
            if let Some(slot) = slot {
                slot.id_name == item.id_name
                    && slot.item_type == item.item_type
                    && slot.check_stackable()
                    && !slot.is_full()
            } else {
                false
            }
        }) {
            if let Some(slot) = slot {
                let remaining: usize = slot.add_to_exist(item.count);
                
                item.count = remaining;

                if remaining > 0 {
                    if let Some(slot) = self.items.iter_mut().find(|slot| {
                        if let Some(slot) = slot {
                            slot.id_name == item.id_name 
                                && slot.item_type == item.item_type
                                && slot.check_stackable()
                                && !slot.is_full()
                        } else {
                            false
                        }
                    }) {
                        if let Some(slot) = slot {
                            let remaining: usize = slot.add_to_exist(remaining);
                            
                            item.count = remaining;
                        }
                    }
                }
            }
        } else {
            if self.add_to_empty_slot(item) {
                item.count = 0;

                return true;
            }
        }
        item.count == 0
    }

    pub fn add_ex(&mut self, item: ItemEntity) -> bool {
        // Если слот с таким именем не найден, ищем свободный слот
        self.add_to_empty_slot(&item)
    }

    /// Добавление в свободный слот
    fn add_to_empty_slot(&mut self, item: &ItemEntity) -> bool {
        if let Some(slot) = self.items.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(Slot {
                name:       item.name.clone(),
                id_name:    item.id_name.clone(),
                id_source:  item.id_source.clone(),
                item_type:  item.item_type.clone(),
                durability: item.durability,
                stack_size: item.stack_size,
                stackable:  item.stackable,
                count:      item.count,
            });

            return true;
        }

        false
    }

    /// Взятие из инвентаря (без выхода)
    pub fn take(&mut self, item: (String, usize)) -> bool {
        if let Some(slot_op) = self.items.iter_mut().find(|slot| {
            if let Some(slot) = slot {
                slot.id_name == item.0 && slot.count >= item.1
            } else {
                false
            }
        }) {
            if let Some(slot) = slot_op {
                if slot.count > item.1 {
                    slot.count -= item.1;
                } else {
                    // *slot = Slot {
                    //     id_name: "".to_string(),
                    //     name: "".to_string(),
                    //     item_type: ItemType::None,
                    //     count: 0,
                    // };
                    *slot_op = None
                }
                return true;
            }
        }
        
        false
    }

    /// Взяти из инвентаря (с выходом)
    pub fn take_all_ex(&mut self, name: &str) -> Option<Slot> {
        if let Some(slot) = self.items.iter_mut().find(|slot_find| {
            if let Some(slot_find) = slot_find {
                slot_find.id_name == name || slot_find.name == name
            } else {
                false
            }
        }) {
            return slot.take();
        }

        None
    }

    /// Поиск в инвентаре
    pub fn find(&mut self, name: &str) -> bool {
        if let Some(_slot) = self.items.iter_mut().find(|slot_find| {
            if let Some(slot_find) = slot_find {
                slot_find.id_name == name || slot_find.name == name
            } else {
                false
            }
        }) {
            return true;
        }

        false
    }

    /// 
    pub fn find_slot_index(&self, name: &str) -> Option<usize> {
        self.items.iter().position(|slot| {
            if let Some(slot) = slot {
                slot.name == name
            } else {
                false
            }
        })
    }

    /// Проверка всех слотов и возвращение предметов
    pub fn iter_some(&self) -> impl Iterator<Item = &Slot> + '_ {
        self.items.iter().filter_map(|slot| slot.as_ref())
    }

    /// Проверка, полон ли инвентарь
    pub fn is_full(&self) -> bool {
        self.items.iter().all(|slot| slot.is_some())
    }

    /// Размер занимаемого пространства
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Проверка, пуст ли инвентарь
    pub fn is_empty(&self) -> bool {
        self.items.iter().all(|slot| slot.is_none())
    }

    // ==========
    // Test
    // ==========
    pub fn get_slot(&self, index: usize) -> Option<&Option<Slot>> {
        self.items.get(index)
    }

    pub fn get_slot_mut(&mut self, index: usize) -> Option<&mut Option<Slot>> {
        self.items.get_mut(index)
    }

    pub fn add_to_slot(&mut self, index: usize, slot: Option<Slot>) {
        if let Some(s) = self.items.get_mut(index) {
            *s = slot;
        }
    }
}

impl Index<usize> for Inventory {
    type Output = Option<Slot>;

    /// Возвращает элемент по указанному индексу
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}