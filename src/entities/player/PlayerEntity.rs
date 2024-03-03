use bevy::prelude::*;

use crate::core::AppState;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct PlayerEntity {
    pub speed: f32
}

impl Default for PlayerEntity {
    fn default() -> Self {
        Self { speed: 25.0 }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Setup), spawn_player)
        .add_systems(Update, player_movement);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("mob.png"),
            ..default()
        },
        PlayerEntity::default()
    ));
}

fn player_movement(
    mut player_entity: Query<(&mut Transform, &PlayerEntity)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut player_entity {
        let movement_amount = player.speed * time.delta_seconds();

        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.y += movement_amount;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.y -= movement_amount;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= movement_amount;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += movement_amount;
        }
    }
}