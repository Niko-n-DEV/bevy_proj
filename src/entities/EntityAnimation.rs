use bevy::prelude::*;

#[derive(Component, Default, Debug, Reflect, PartialEq, Clone, Copy)]
pub enum EntityDirectionState {
    #[default]
    South   = 0,
    North   = 1,
    East    = 2,
    West    = 3,
}

impl EntityDirectionState {
    pub fn dir_index(&self) -> usize {
        *self as usize
    }

    pub fn calculate_index(
        atlas_number: usize, 
        direction_index: usize
    ) -> usize {
        atlas_number * 4 + direction_index
    }
}

#[derive(Component, Default, Debug, Reflect, PartialEq, Clone, Copy)]
pub enum EntityEightDirectionState {
    NorthWest   = 0,
    North       = 1,
    NorthEast   = 2,
    West        = 3,
    None        = 4,
    East        = 5,
    SouthWest   = 6,
    #[default]
    South       = 7,
    SouthEast   = 8,
}

#[allow(unused)]
impl EntityEightDirectionState {
    pub fn dir_index(&self) -> usize {
        *self as usize
    }

    pub fn calculate_index(
        atlas_size: usize,
        atlas_number: usize, 
        direction_index: usize
    ) -> usize {
        let group_width = (atlas_size / 3).min(atlas_size);
        
        // Определяем координаты группы
        let group_x = atlas_number % (atlas_size / group_width);
        let group_y = atlas_number / (atlas_size / group_width);

        // Определяем координаты внутри группы
        let texture_x = direction_index % 3;
        let texture_y = direction_index / 3;

        // Конечные глобальные координаты
        let global_x = group_x * 3 + texture_x;
        let global_y = group_y * 3 + texture_y;

        // Возвращаем индекс как одномерный
        global_y * atlas_size + global_x
    }
}