use bevy::prelude::*;

mod components;
mod entities;
mod systems;
mod util;

mod core {
    #![allow(non_snake_case)]

    pub use crate::AppState;

    pub use crate::entities::player::PlayerEntity::*;
    pub use crate::components::Camera::*;

    #[allow(unused_imports)]
    pub use crate::systems::*;

    //pub use crate::systems::graphic::*;

    pub use crate::systems::interface::*;

    pub use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    pub use serde_json::*;
    #[allow(unused_imports)]
    pub use std::fs;
    #[allow(unused_imports)]
    pub use std::fs::File;
    #[allow(unused_imports)]
    pub use std::path::Path;
    #[allow(unused_imports)]
    pub use std::fs::Metadata;
    #[allow(unused_imports)]
    pub use std::io::{Read, Write};
    #[allow(unused_imports)]
    pub use crate::util::*;
}

use core::UI::UI;

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
    .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)))
    .add_plugins((CameraController, UI))
    .add_plugins(Player)
    .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Setup,
    ResourceLoading,
    #[default]
    MainMenu,
    Game,
    Pause,
    Finished
}