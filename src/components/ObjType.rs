#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    resource::graphic::Atlas::TestTextureAtlas,
    AppState,
    ItemType::{ItemType, Pickupable},
};

use serde::Deserialize;

use bevy_inspector_egui::InspectorOptions;

/// Перечисление того, что является объектом мира
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum WorldObject {
    Item(ItemType),
    Plant(PlantType),
}

impl WorldObject {
    pub fn spawn(self, commands: &mut Commands, graphics: &TestTextureAtlas, position: Vec2) {
        // -> Entity {
        // let sprite = graphics
        //     .item_map
        //     .get(&self)
        //     .expect(&format!("No graphic for object {:?}", self))
        //     .clone();

        // let item = commands
        //     .spawn_bundle(SpriteSheetBundle {
        //         texture: graphics.image.clone(),
        //         atlas: graphics.layout.clone(),
        //         transform: Transform {
        //             translation: position.extend(0.0),
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     })
        //     .insert(Name::new("GroundItem"))
        //     .insert(self)
        //     .id();

        // if let Some(harvest) = self.as_harvest() {
        //     commands.entity(item).insert(harvest);
        // }

        // if let Some(pickup) = self.as_pickup() {
        //     commands.entity(item).insert(pickup);
        // }

        // if self.grows_into().is_some() {
        //     commands.entity(item).insert(GrowthTimer {
        //         timer: Timer::from_seconds(3.0, false),
        //     });
        // }

        // item
    }

    pub fn grow(
        self,
        commands: &mut Commands,
        graphics: &TestTextureAtlas,
        ent: Entity,
        transform: &Transform,
    ) { // -> Entity {
         // if let Some(new_object) = self.grows_into() {
         //     commands.entity(ent).despawn_recursive();
         //     new_object.spawn(commands, graphics, transform.translation.truncate())
         //     //println!("{:?} grew into a beautiful {:?}", self, self.grows_into());
         // } else {
         //     ent
         // }
    }

    /// TODO it would be great to describe this outside of code, in a config or something
    pub fn grows_into(&self) -> Option<WorldObject> {
        match self {
            // WorldObject::DeadSapling => Some(WorldObject::Sapling),
            // WorldObject::PluckedGrass => Some(WorldObject::Grass),
            // WorldObject::GrowingTree => Some(WorldObject::Tree),
            _ => None,
        }
    }

    /// TODO it would be great to describe this outside of code, in a config or something
    pub fn as_harvest(&self) -> Option<Harvestable> {
        match self {
            // WorldObject::Sapling => Some(Harvestable {
            //     item: ItemType::Twig,
            //     tool_required: None,
            //     drops: Some(WorldObject::DeadSapling),
            // }),
            // WorldObject::Grass => Some(Harvestable {
            //     item: ItemType::Grass,
            //     tool_required: None,
            //     drops: Some(WorldObject::PluckedGrass),
            // }),
            // WorldObject::Tree => Some(Harvestable {
            //     item: ItemType::Wood,
            //     tool_required: Some(Tool::Axe),
            //     drops: Some(WorldObject::Stump),
            // }),
            _ => None,
        }
    }

    pub fn as_pickup(&self) -> Option<Pickupable> {
        if self.as_harvest().is_some() {
            return None;
        }
        match self {
            WorldObject::Item(item) => Some(Pickupable { item: *item }),
            _ => None,
        }
    }
}

impl Default for WorldObject {
    fn default() -> Self {
        WorldObject::Item(ItemType::None)
    }
}

/// Перечисление того, что является растением
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum PlantType {
    Grass,
    Tree,
    Bush,
    Flower,
    Thickets,
    Vegetation,
}

/// Структура для того, что может быть собрано
#[derive(Component)]
pub struct Harvestable {
    //pub(crate) item: ItemType,
    //pub(crate) tool_required: Option<Tool>,
    pub(crate) drops: Option<WorldObject>,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GrowthTimer {
    timer: Timer,
}
