use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::core::{items::Weapon::*, AppState, Input::OffsetedCursorPosition, bullet::*, Entity::update_enemies, entities::EntitySys::{update_spawning, EnemySpawner}};

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct PlayerEntity {
    pub speed: f32,
    pub sprint: f32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub movable: bool,
}

impl Default for PlayerEntity {
    fn default() -> Self {
        Self {
            speed: 25.0,
            sprint: 50.0,
            position: Vec3::new(126.0, -126.0, 0.0),
            velocity: Vec3::ZERO,
            movable: true,
        }
    }
}

impl PlayerEntity {}

pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), Self::spawn_player)
            .register_type::<PlayerEntity>()
            .insert_resource(OffsetedCursorPosition {x: 0., y: 0.})
            .add_systems(Update, Self::player_movement.run_if(in_state(AppState::Game)))
            .add_systems(Update, gun_controls.run_if(in_state(AppState::Game)))
            .add_systems(Update, attach_objects.run_if(in_state(AppState::Game)))
            .add_systems(Update, 
                (
                            update_bullets.run_if(in_state(AppState::Game)), 
                            update_bullet_hits.run_if(in_state(AppState::Game))
                        )
                    )
            .add_systems(Update, 
                (
                            update_enemies.run_if(in_state(AppState::Game)), 
                            update_spawning.run_if(in_state(AppState::Game))
                        )
                    )
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
            Name::new("Player"),
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: asset_server.load("gun.png"),
                transform: Transform {
                    translation: Vec3::splat(0.),
                    ..default()
                },
                ..default()
            }).insert(PlayerAttach{ offset: Vec2::new(0.,0.)}).insert(GunController{ shoot_cooldown: 0.3,shoot_timer: 0.});
        });

        //commands.spawn(TransformBundle{..default()}).insert(EnemySpawner{cooldown: 1., timer: 1.});
    }

    fn despawn_player(mut commands: Commands, player: Query<Entity, With<PlayerEntity>>) {
        if let Ok(player) = player.get_single() {
            commands.entity(player).despawn_recursive()
        }
    }

    fn player_movement(
        // mut commands: Commands,
        // asset_server: Res<AssetServer>,
        mut player_entity: Query<(&mut Transform, &mut PlayerEntity)>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        _mouse_input: Res<OffsetedCursorPosition>,
        time: Res<Time>,
    ) {
        for (mut transform, mut player) in &mut player_entity {
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
                    let new_pos = transform.translation
                        + time.delta_seconds() * speed_var * direction.normalize();
                    transform.translation = new_pos;
                    player.position = new_pos;
                } else {
                    let pos = player.position;
                    transform.translation = pos
                }
            }
        }
    }
}

#[derive(Component, InspectorOptions)]
pub struct Inventory {}

#[derive(Component)]
pub struct PlayerAttach {
    pub offset: Vec2,
}

pub fn attach_objects(
    player_query: Query<(&PlayerEntity, &mut Transform), Without<PlayerAttach>>,
    mut objects_query: Query<(&PlayerAttach, &mut Transform), Without<PlayerEntity>>,
) {
    if let Ok((_, player_transform)) = player_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            transform.translation =
                player_transform.translation + Vec3::new(attach.offset.x, attach.offset.y, 1.);
        }
    }
}
