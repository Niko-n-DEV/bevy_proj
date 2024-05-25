use bevy::{
    audio::{AudioPlugin, SpatialScale},
    asset::io::{
        file::FileAssetReader, 
        AssetSource
    }, 
    prelude::*
};

// Определение модулей

mod components;
mod entities;
mod items;
mod objects;
mod systems;
mod util;

/// Совокупность основных модулей, именуемое как `core` , для предоставления быстрого доступа ко всем модулям
mod core {
    #![allow(non_snake_case)]
    pub use crate::AppState;

    pub use crate::components::*;
    pub use crate::objects::*;
    pub use crate::items::*;
    pub use crate::entities::*;
    pub use crate::systems::*;

    pub use crate::util::*;
}

use crate::core::{
    resource::ResourcePlugin,
    world::World::WorldSystem,
    Camera::CameraController,
    interface::UIPlugin,
    UserSystem::UserPlugin
};

use bevy::{input::common_conditions::input_toggle_active, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use iyes_perf_ui::prelude::*;

const AUDIO_SCALE: f32 = 1. / 32.0;

fn main() {
    App::new()
        // Init State - Инициализация состояний приложения
        .init_state::<AppState>()
        // Init Resources - Установка фонового цвета приложения
        .insert_resource(ClearColor(Color::rgb_u8(31, 31, 31)))
        // Инициализация базового плагина, отвечающего за создание окна приложения с определённой конфигурацией
        // Имя окна - SINT-et
        // [Test] Стартовое разрешение окна - 1280 на 720
        // Есть возможность изменения размера окна
        // [Test] Включена Вертикальная синхронизация (у меня 144 герц) [AutoVsync]
        // Установлен nearest фильтр, чтобы спрайты были несглаженные.
        .register_asset_source("Data", AssetSource::build()
            .with_reader(|| Box::new(FileAssetReader::new("Data")))
        )
        .add_plugins(DefaultPlugins
                .set(
                    WindowPlugin {
                        primary_window: Some(Window {
                            title: "SINT-et".to_string(),
                            resolution: WindowResolution::new(1280.0, 720.0),
                            resizable: false,
                            present_mode: bevy::window::PresentMode::AutoVsync,
                            ..default()
                        }),
                        ..default()
                    }
                )
                .set(AudioPlugin {
                    default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build())
        .insert_resource(Msaa::Off)
        // Плагин - Инспектор, для отладки и мониторинга элементов
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F9)),
        )
        // Плагины для Debug info panel
        .add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            PerfUiPlugin,
        ))
        // Инициализация основных плагинов приложения
        // Инициализация загрузки ресурсов приложения
        .add_plugins(ResourcePlugin)
        .add_plugins((
            CameraController,   // Инициализация плагина камеры
            UIPlugin,           // Инициализация плагина пользовательского графического интерфейса
            UserPlugin          // Инициализация интерфейса взаимодействия пользователя
        ))
        .add_plugins(WorldSystem)
        // Инициализация StartUP функции setup
        .add_systems(Startup, setup)
        .run();
}

// #[derive(Component)]
// struct UiInfo;

// [Test] Debug info panel
fn setup(mut _commands: Commands) {
    if true {
        _commands.spawn((
            PerfUiRoot::default(),
            PerfUiEntryFPS::default(),
            PerfUiEntryFPSWorst::default(),
            PerfUiEntryFrameTime::default(),
            PerfUiEntryFrameTimeWorst::default(),
            PerfUiEntryFrameCount::default(),
            PerfUiEntryEntityCount::default(),
            PerfUiEntryRunningTime::default(),
            PerfUiEntryClock::default(),
        ));
    }
}

/// Состояние приложения
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Start,              // Инициализация программы
    #[default]
    ResourceCheck,      // Этап проверки ресурсов
    ResourceLoading,    // Этап загрузки ресурсов
    MainMenu,           // Этап главного меню
    LoadingInGame,      // Этап загрузки мира
    Game,               // Этап игрового процесса
    Pause,              // Пауза
    SavingGame,         // Этап сохранения данных мира (при сохранении мир находится в состоянии Pause, может быть обратно переведён в game)
    Finished,           // Этап полного сохранения данных и переход в главное меню/закрытие программы
}
