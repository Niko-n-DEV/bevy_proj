use bevy::prelude::*;

#[derive(Component)]
struct Velocity {
    pub value: Vec2
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        
    }
}