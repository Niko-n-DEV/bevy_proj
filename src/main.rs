use bevy::prelude::*;

use bevy::{input::common_conditions::input_toggle_active, window::WindowResolution};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod components;
mod entities;
mod systems;
mod util;

mod core {
    #![allow(non_snake_case)]

    pub use crate::AppState;

    pub use crate::components::*;
    pub use crate::entities::*;
    pub use crate::systems::*;

    //pub use crate::systems::graphic::*;

    pub use crate::systems::interface::*;

    pub use crate::util::*;
}

use crate::core::{
    player::PlayerEntity::Player, world::World::WorldSystem, Camera::CameraController, UI::UI,
};

fn main() {
    App::new()
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::rgb_u8(31, 31, 31)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Test".to_string(),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        resizable: true,
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build(),
            EguiPlugin,
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)),
        )
        .add_plugins((CameraController, UI))
        .add_plugins((Player, WorldSystem))
        .add_systems(OnEnter(AppState::Game), setup)
        .run();
}

fn setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Start,
    ResourceLoading,
    ResourceCorrect,
    #[default]
    MainMenu,
    Game,
    Pause,
    Finished,
}
