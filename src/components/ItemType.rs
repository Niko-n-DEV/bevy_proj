#[allow(unused)]
use bevy::prelude::*;

use serde::Deserialize;

use bevy_inspector_egui::InspectorOptions;

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

/// Всё, что может быть взято как иструмент
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum Tool {
    Axe,
    Shovel,
    Hoe,
    Pickaxe,
}

/// Всё, что может быть взято как оружие
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum Weapon {
    Gun,
}

/// Всё, что является предметом
#[derive(InspectorOptions, Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum Item {
    Material(Material),
    Consumables,
    Ammo,
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
    Rocket,
}

/// Структура для того, что может быть поднято
#[derive(Component, InspectorOptions)]
pub struct Pickupable {
    pub(crate) item: ItemType,
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
