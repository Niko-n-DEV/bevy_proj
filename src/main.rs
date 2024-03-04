mod components;
mod entities;
mod systems;

mod core {
    #![allow(non_snake_case)]
    pub use bevy::prelude::*;

    pub use crate::AppState;

    pub use crate::entities::player::PlayerEntity::*;
    pub use crate::components::Camera::*;

    //pub use crate::systems::graphic::*;

    pub use crate::systems::interface::*;
}

use core::UI::GameUI;

use bevy::{input::common_conditions::input_toggle_active, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::core::*;

fn main() {
    App::new()
    .init_state::<AppState>()
    .insert_resource(ClearColor(Color::rgb_u8(50, 50, 50)))
    .add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test".to_string(),
                resolution: WindowResolution::new(1280.0, 720.0),
                resizable: true,
                ..default()
            }),
            ..default()
            })
        .build()
    )
    .add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
    )
    .add_plugins((CameraControllerPlugin, GameUI))
    .add_plugins(PlayerPlugin)
    .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Setup,
    ResourceLoading,
    MainMenu,
    Game,
    Pause,
    Finished
}