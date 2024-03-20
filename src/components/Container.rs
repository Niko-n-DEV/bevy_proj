use bevy::prelude::*;

#[derive(Component)]
pub struct Container {
    pub size: i32
}

impl Default for Container {
    fn default() -> Self {
        Self {
            size: 1
        }
    }
}