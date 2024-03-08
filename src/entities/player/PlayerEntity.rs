
#[allow(unused_imports)]
use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::AppState;

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct PlayerEntity {
    pub speed: f32,
    pub sprint: f32,
    pub movable: bool,
}

impl Default for PlayerEntity {
    fn default() -> Self {
        Self {
            speed: 25.0,
            sprint: 50.0,
            movable: true,
        }
    }
}

pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), Self::spawn_player)
            .add_systems(Update, Self::player_movement.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), Self::despawn_player);
    }
}

impl Player {
    fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("mob.png"),
                ..default()
            },
            PlayerEntity::default(),
            Name::new("Player")
        ));
    }

    fn despawn_player(mut commands: Commands, player: Query<Entity, With<PlayerEntity>>) {
        if let Ok(player) = player.get_single() {
            commands.entity(player).despawn_recursive()
        }
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
                    transform.translation +=
                        time.delta_seconds() * speed_var * direction.normalize();
                }
            }
        }
    }
}
