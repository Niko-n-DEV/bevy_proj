#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    Object::EntityObject,
    world::Grid::Grid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum ConnectedSprite {
    All,
    None,
    NorthSouth,
    EastWest,
    East,
    South,
    West,
    North,
    EastSouth,
    WestSouth,
    NorthWest,
    NorthEast,
    NorthWestSouth,
    NorthEastWest,
    NorthEastSouth,
    EastWestSouth,
    Outline,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub enum Facing {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

impl Facing {
    //Gross but works ugh
    pub fn from_direction(direction: &Vec2) -> Self {
        if direction.x == 0.0 && direction.y == 0.0 {
            Facing::Down
        } else if direction.x.abs() > direction.y.abs() {
            if direction.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            }
        } else if direction.y > 0.0 {
            Facing::Up
        } else {
            Facing::Down
        }
    }
}

// impl IndexableSprite for WallSprite {
//     type AtlasHandleWrapper = CharacterAtlas;
//     fn index(&self) -> usize {
//         match self {
//             ConnectedSprite::All => 12,
//             ConnectedSprite::None => 13,
//             ConnectedSprite::NorthSouth => 14,
//             ConnectedSprite::EastWest => 15,

//             ConnectedSprite::East => 12 + 16,
//             ConnectedSprite::South => 13 + 16,
//             ConnectedSprite::West => 14 + 16,
//             ConnectedSprite::North => 15 + 16,

//             ConnectedSprite::EastSouth => 12 + 16 * 2,
//             ConnectedSprite::WestSouth => 13 + 16 * 2,
//             ConnectedSprite::NorthWest => 14 + 16 * 2,
//             ConnectedSprite::NorthEast => 15 + 16 * 2,

//             ConnectedSprite::NorthWestSouth => 12 + 16 * 3,
//             ConnectedSprite::NorthEastWest => 13 + 16 * 3,
//             ConnectedSprite::NorthEastSouth => 14 + 16 * 3,
//             ConnectedSprite::EastWestSouth => 15 + 16 * 3,

//             ConnectedSprite::Outline => 15 + 16 * 4,
//         }
//     }
// }

fn update_wall_sprite(mut sprites: Query<&mut ConnectedSprite>, grid: Res<Grid<EntityObject>>) {
    // for (ent, location) in grid.iter() {
    //     if let Ok(mut wall) = sprites.get_mut(ent) {
    //         let east = &(location.0 + IVec2::new(1, 0)).into();
    //         let west = &(location.0 - IVec2::new(1, 0)).into();
    //         let north = &(location.0 + IVec2::new(0, 1)).into();
    //         let south = &(location.0 - IVec2::new(0, 1)).into();

    //         use ConnectedSprite::*;
    //         *wall = match (
    //             grid.occupied(west),
    //             grid.occupied(east),
    //             grid.occupied(north),
    //             grid.occupied(south),
    //         ) {
    //             (true, true, true, true) => All,
    //             (true, true, true, false) => NorthEastWest,
    //             (true, true, false, true) => EastWestSouth,
    //             (true, true, false, false) => EastWest,
    //             (true, false, true, true) => NorthWestSouth,
    //             (true, false, true, false) => NorthWest,
    //             (true, false, false, true) => WestSouth,
    //             (true, false, false, false) => West,
    //             (false, true, true, true) => NorthEastSouth,
    //             (false, true, true, false) => NorthEast,
    //             (false, true, false, true) => EastSouth,
    //             (false, true, false, false) => East,
    //             (false, false, true, true) => NorthSouth,
    //             (false, false, true, false) => North,
    //             (false, false, false, true) => South,
    //             (false, false, false, false) => None,
    //         };
    //     }
    // }
}