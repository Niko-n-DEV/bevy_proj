mod components;
mod entities;
mod systems;

mod core {
    #![allow(non_snake_case)]
    pub use bevy::prelude::*;

    pub use crate::entities::player::*;
    pub use crate::components::Camera::*;
    
}

use bevy::window::WindowResolution;

use crate::core::*;

fn main() {
    App::new()
        .add_systems(PreStartup, setup)
        .add_plugins(CameraControllerPlugin)
    .insert_resource(ClearColor(Color::rgb_u8(50, 50, 50)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Test".to_string(),
            resolution: WindowResolution::new(1280.0, 720.0),
            resizable: true,
            
            ..default()
        }),
        ..default()
    }))
    
    .run();
}

fn setup(mut commands: Commands) {
    // Все процессы загрузки, таких как парсинг настроек, конфигов, модов и т.п.
}
