#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    graphic::Atlas::TestTextureAtlas,
    AppState
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

    pub fn spawn(self, commands: &mut Commands, graphics: &TestTextureAtlas, position: Vec2) { // -> Entity {
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
    Vegetation
}

/// Всё, что может бить предметом 
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum ItemType {
    None,
    Tool(Tool),
    Weapon(Weapon),
    Item(Item),
}

impl ItemType {
    #[allow(dead_code)]
    /// Получение имени типа
    pub fn name(self) -> String {
        match self {
            ItemType::Tool(tool) => format!("{:?}", tool),
            _ => format!("{:?}", self),
        }
    }
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::None
    }
}

/// Структура для того, что может быть собрано
#[derive(Component)]
pub struct Harvestable {
    //pub(crate) item: ItemType,
    //pub(crate) tool_required: Option<Tool>,
    pub(crate) drops: Option<WorldObject>,
}

/// Структура для того, что может быть поднято
#[derive(Component, InspectorOptions)]
pub struct Pickupable {
    pub(crate) item: ItemType,
}

/// Всё, что может быть взято как иструмент
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum Tool {
    Axe,
    Shovel,
    Hoe,
    Pickaxe
}

/// Всё, что может быть взято как оружие
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum Weapon {
    Gun
}

/// Всё, что является предметом
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum Item {
    Material(Material),
    Consumables,
    Ammo
}

/// Перечисление того, что является предметом
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum Material {
    Wood,
    Cobblestone,
    Flint,
}

/// Перечисление того, что является боеприпасом
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum Ammo {
    SmallAmmo,
    MediumAmmo,
    LargeAmmo,
    Rocket
}

#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub struct ItemAndCount {
    pub item: ItemType,
    pub count: usize,
}

impl std::fmt::Display for ItemAndCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x {:?}", self.count, self.item)
    }
}

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        // app
        //     // .add_systems(
        //     //     OnEnter(AppState::Game),
                
        //     //     //.with_system(Self::spawn_test_objects.after("graphics")),
        //     // )
        // .add_systems(
        //     SystemSet::on_update(GameState::Main)
        //         .with_system(Self::update_graphics)
        //         .with_system(Self::world_object_growth),
        // );
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GrowthTimer {
    timer: Timer,
}

impl ItemsPlugin {
    // Ticks the timers for everything in the world that can regrow and calls grow on them
    // fn world_object_growth(
    //     mut commands: Commands,
    //     time: Res<Time>,
    //     graphics: Res<Graphics>,
    //     mut growable_query: Query<(Entity, &Transform, &WorldObject, Option<&mut GrowthTimer>)>,
    // ) {
    //     for (ent, transform, world_object, regrowth_timer) in growable_query.iter_mut() {
    //         if let Some(mut timer) = regrowth_timer {
    //             timer.timer.tick(time.delta());
    //             if !timer.timer.finished() {
    //                 continue;
    //             }

    //             world_object.grow(&mut commands, &graphics, ent, transform);
    //         }
    //     }
    // }

    // Keeps the graphics up to date for things that are harvested or grown
    // fn update_graphics(
    //     mut to_update_query: Query<(&mut TextureAtlasSprite, &WorldObject), Changed<WorldObject>>,
    //     graphics: Res<Graphics>,
    // ) {
    //     for (mut sprite, world_object) in to_update_query.iter_mut() {
    //         sprite.clone_from(
    //             graphics
    //                 .item_map
    //                 .get(world_object)
    //                 .expect(&format!("No graphic for object {:?}", world_object)),
    //         );
    //     }
    // }

    // /// Creates our testing map
    // #[allow(clippy::vec_init_then_push)]
    // fn spawn_test_objects(mut commands: Commands, graphics: Res<Graphics>) {
    //     let mut children = Vec::new();
    //     children.push(WorldObject::Sapling.spawn(&mut commands, &graphics, Vec2::new(-3., 3.)));
    //     children.push(WorldObject::Sapling.spawn(&mut commands, &graphics, Vec2::new(-3., 1.)));
    //     children.push(WorldObject::Sapling.spawn(&mut commands, &graphics, Vec2::new(-1., 3.)));
    //     children.push(WorldObject::Sapling.spawn(&mut commands, &graphics, Vec2::new(-1., 1.)));

    //     children.push(WorldObject::Grass.spawn(&mut commands, &graphics, Vec2::new(3., -3.)));
    //     children.push(WorldObject::Grass.spawn(&mut commands, &graphics, Vec2::new(3., -1.)));
    //     children.push(WorldObject::Grass.spawn(&mut commands, &graphics, Vec2::new(1., -3.)));
    //     children.push(WorldObject::Grass.spawn(&mut commands, &graphics, Vec2::new(1., -1.)));

    //     children.push(WorldObject::Tree.spawn(&mut commands, &graphics, Vec2::new(-3., -3.)));
    //     children.push(WorldObject::Tree.spawn(&mut commands, &graphics, Vec2::new(-3., -1.)));
    //     children.push(WorldObject::Tree.spawn(&mut commands, &graphics, Vec2::new(-1., -3.)));
    //     children.push(WorldObject::Tree.spawn(&mut commands, &graphics, Vec2::new(-1., -1.)));

    //     children.push(WorldObject::Item(ItemType::Flint).spawn(
    //         &mut commands,
    //         &graphics,
    //         Vec2::new(3., 3.),
    //     ));
    //     children.push(WorldObject::Item(ItemType::Flint).spawn(
    //         &mut commands,
    //         &graphics,
    //         Vec2::new(3., 1.),
    //     ));
    //     children.push(WorldObject::Item(ItemType::Flint).spawn(
    //         &mut commands,
    //         &graphics,
    //         Vec2::new(1., 3.),
    //     ));
    //     children.push(WorldObject::Item(ItemType::Flint).spawn(
    //         &mut commands,
    //         &graphics,
    //         Vec2::new(1., 1.),
    //     ));
    //     commands
    //         .spawn_bundle(TransformBundle::default())
    //         .insert(Name::new("Test Objects"))
    //         .push_children(&children);
    // }
}