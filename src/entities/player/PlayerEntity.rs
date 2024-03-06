#[allow(unused_imports)]
use bevy::prelude::*;
use crate::core::AppState;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct PlayerEntity {
    pub speed: f32,
    pub sprint: f32,
    pub movable: bool
}

impl Default for PlayerEntity {
    fn default() -> Self {
        Self { 
            speed: 25.0,
            sprint: 50.0,
            movable: true
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Game), spawn_player)
        .add_systems(Update, player_movement.run_if(in_state(AppState::Game)));
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
        if player.movable {
            let mut direction = Vec3::ZERO;
            let mut speed_var: f32 = player.speed;

            if keyboard_input.pressed(KeyCode::ShiftLeft) {
                speed_var = player.sprint;
            }
            
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction.y += 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction.y -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                direction.x -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                direction.x += 1.0;
            }

            if direction != Vec3::ZERO {
                transform.translation += time.delta_seconds() * speed_var * direction.normalize();
            }
        }
    }
}