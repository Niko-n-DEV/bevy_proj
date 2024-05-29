#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    Object::PersistentObject,
    world::Grid::Grid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum ConnectedObject {
    All             = 0,
    None            = 1,
    NorthSouth      = 2,
    EastWest        = 3,
    East            = 4,
    South           = 5,
    West            = 6,
    North           = 7,
    EastSouth       = 8,
    WestSouth       = 9,
    NorthWest       = 10,
    NorthEast       = 11,
    NorthWestSouth  = 12,
    NorthEastWest   = 13,
    NorthEastSouth  = 14,
    EastWestSouth   = 15,
    // Outline = 16,
}

impl ConnectedObject {
    pub fn dir_index(&self) -> usize {
        *self as usize
    }

    pub fn calculate_index(
        atlas_size: usize,
        atlas_number: usize, 
        direction_index: usize
    ) -> usize {
        let group_width = (atlas_size / 4).min(atlas_size);
        
        // Определяем координаты группы
        let group_x = atlas_number % (atlas_size / group_width);
        let group_y = atlas_number / (atlas_size / group_width);

        // Определяем координаты внутри группы
        let texture_x = direction_index % 4;
        let texture_y = direction_index / 4;

        // Конечные глобальные координаты
        let global_x = group_x * 4 + texture_x;
        let global_y = group_y * 4 + texture_y;

        // Возвращаем индекс как одномерный
        global_y * atlas_size + global_x
    }
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

// fn update_wall_sprite(mut sprites: Query<&mut ConnectedObject>, grid: Res<Grid<PersistentObject>>) {
//     for (ent, location) in grid {
//         if let Ok(mut wall) = sprites.get_mut(ent) {
//             let east = &(location.0 + IVec2::new(1, 0)).into();
//             let west = &(location.0 - IVec2::new(1, 0)).into();
//             let north = &(location.0 + IVec2::new(0, 1)).into();
//             let south = &(location.0 - IVec2::new(0, 1)).into();

//             use ConnectedObject::*;
//             *wall = match (
//                 grid.occupied(west),
//                 grid.occupied(east),
//                 grid.occupied(north),
//                 grid.occupied(south),
//             ) {
//                 (true, true, true, true)     => All,
//                 (true, true, true, false)    => NorthEastWest,
//                 (true, true, false, true)    => EastWestSouth,
//                 (true, true, false, false)   => EastWest,
//                 (true, false, true, true)    => NorthWestSouth,
//                 (true, false, true, false)   => NorthWest,
//                 (true, false, false, true)   => WestSouth,
//                 (true, false, false, false)  => West,
//                 (false, true, true, true)    => NorthEastSouth,
//                 (false, true, true, false)   => NorthEast,
//                 (false, true, false, true)   => EastSouth,
//                 (false, true, false, false)  => East,
//                 (false, false, true, true)   => NorthSouth,
//                 (false, false, true, false)  => North,
//                 (false, false, false, true)  => South,
//                 (false, false, false, false) => None,
//             };
//         }
//     }
// }