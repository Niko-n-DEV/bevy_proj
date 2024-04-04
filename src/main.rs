use core::graphic::Graphic::Graphic;

use bevy::prelude::*;

use bevy::{input::common_conditions::input_toggle_active, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_pancam::PanCamPlugin;

use iyes_perf_ui::prelude::*;

// Определение модулей

mod components;
mod entities;
mod systems;
mod util;

/// Совокупность основных модулей, именуемое как `core` , для предоставления быстрого доступа ко всем модулям
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
    player::PlayerEntity::PlayerPlugin, 
    entities::EntitySystem::EntitySystem,
    world::World::WorldSystem, 
    Camera::CameraController, 
    UI::UI //, world::TileMap::TileMapPlugin
};

fn main() {
    App::new()
        // Инициализация состояний приложения
        .init_state::<AppState>()
        // Установка фонового цвета приложения
        .insert_resource(ClearColor(Color::rgb_u8(31, 31, 31)))
        // Инициализация базового плагина, отвечающего за создание окна приложения с определённой конфигурацией
        // Имя окна - SINT-et
        // [Test] Стартовое разрешение окна - 1280 на 720
        // Есть возможность изменения размера окна
        // [Test] Включена Вертикальная синхронизация (у меня 144 герц) [AutoVsync]
        // Установлен nearest фильтр, чтобы спрайты были несглаженные.
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SINT-et".to_string(),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        resizable: true,
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build(),
                PanCamPlugin::default()
        ))
        // Плагин - Инспектор, для отладки и мониторинга элементов
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)),
        )
        // Плагины для Debug info panel
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        // Инициализация основных плагинов приложения
        // Инициализация загрузки ресурсов приложения
        .add_plugins(Graphic)
        // Инициализация плагина камеры и пользовательского интерфейса
        .add_plugins((CameraController, UI))
        // Инициализация плагина игрока и [Test] Системы Мира
        .add_plugins((EntitySystem, PlayerPlugin, WorldSystem)) //, TileMapPlugin))
        // Инициализация StartUP функции setup
        .add_systems(Startup, setup)
        .run();
}

// #[derive(Component)]
// struct UiInfo;

// [Test] Debug info panel
fn setup(mut _commands: Commands) {
    _commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFrameTime::default(),
        PerfUiEntryFrameTimeWorst::default(),
        PerfUiEntryFrameCount::default(),
        PerfUiEntryEntityCount::default(),
        PerfUiEntryCpuUsage::default(),
        PerfUiEntryMemUsage::default(),
        PerfUiEntryRunningTime::default(),
        PerfUiEntryClock::default(),
    ));
}

/// Состояние приложения
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Start,
    #[default]
    ResourceCheck,
    ResourceLoading,
    MainMenu,
    LoadingInGame,
    Game,
    Pause,
    SavingGame,
    Finished,
}
