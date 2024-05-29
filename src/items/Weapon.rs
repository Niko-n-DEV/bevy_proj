#![allow(unused)]
use bevy::prelude::*;

use crate::core::{
    PlayerSystem::PlayerAttach,
    resource::{
        Registry::Registry,
        graphic::Atlas::AtlasRes
    },
    UserSystem::{
        CursorPosition,
        UserControl,
        User
    },
    Missile::{
        Bullet, 
        BULLET_LIFETIME, 
        BULLET_SPEED
    },
    ContainerSystem::Inventory,
    ItemType::{
        ItemType,
        Item
    }
};

#[derive(Component)]
pub struct GunController {
    pub shoot_cooldown: f32,
    pub shoot_timer: f32,
}

pub fn gun_controls(
    mut commands: Commands,
    mut gun_query: Query<(
        &mut GunController,
        &mut Transform,
        &mut Sprite,
        &mut PlayerAttach,
    )>,
    mut user_container: Query<&mut Inventory, With<UserControl>>,
    cursor:             Res<CursorPosition>,
    time:               Res<Time>,
    _buttons:           Res<ButtonInput<MouseButton>>,
    _keyboard_input:    Res<ButtonInput<KeyCode>>,
    atlas:              Res<AtlasRes>,
    register:           Res<Registry>
) {
    if gun_query.is_empty() || user_container.is_empty() {
        return;
    }

    let mut container = user_container.single_mut();

    for (mut gun_controller, mut transform, mut sprite, mut attach) in gun_query.iter_mut() {
        gun_controller.shoot_timer -= time.delta_seconds();

        let cursor_pos = cursor.0;

        let diff = Vec2::new(
            cursor_pos.x - transform.translation.x,
            cursor_pos.y - transform.translation.y,
        );
        let angle = diff.y.atan2(diff.x);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);

        if cursor_pos.x > transform.translation.x {
            sprite.flip_y = false
        } else {
            sprite.flip_y = true
        }

        if gun_controller.shoot_timer <= 0. {
            if _buttons.pressed(MouseButton::Right) {
                attach.offset = Vec2::new(0., -2.);
                if _buttons.pressed(MouseButton::Left) {
                    
                    let mut ammo_found = false;

                    if container.take(("bullet".to_string(), 1)) {
                        ammo_found = true;
                    }

                    if !ammo_found {
                        return;
                    }

                    let mut spawn_transform = Transform::from_scale(Vec3::splat(1.0));
                    spawn_transform.translation = transform.translation;
                    spawn_transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
                    gun_controller.shoot_timer = gun_controller.shoot_cooldown;

                    if let Some(sprite) = register.get_test("bullet_p", &atlas) {
                        let sprite_ex = (sprite.texture, sprite.atlas);
                        commands
                            .spawn(SpriteSheetBundle {
                                transform:  spawn_transform,
                                texture:    sprite_ex.0,
                                atlas:      sprite_ex.1,
                                ..default()
                            })
                            .insert(Name::new("Bullet"))
                            .insert(Bullet {
                                lifetime:   BULLET_LIFETIME,
                                speed:      BULLET_SPEED,
                                direction:  diff.normalize(),
                            });
                    } else {
                        println!("error")
                    }
                }
            } else {
                attach.offset = Vec2::new(0., -3.);
            }
        }
    }
}

// ==============================
// Melee
// ==============================

#[derive(Component, Debug, Clone)]
pub struct AttackTimer(pub Timer);

#[derive(Component, Reflect, Default, Clone)]
pub struct MeleeAttack;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        // app.with_default_schedule(CoreSchedule::FixedUpdate, |app| {
        //     app.add_event::<HitEvent>().add_event::<EnemyDeathEvent>();
        // })
        // .add_event::<ObjBreakEvent>()
        // .add_plugin(CollisionPlugion)
        // .add_systems(
        //     (
        //         handle_hits,
        //         cleanup_marked_for_death_entities.after(handle_enemy_death),
        //         handle_attack_cooldowns.before(CustomFlush),
        //         // spawn_hit_spark_effect.after(handle_hits),
        //         handle_invincibility_frames.after(handle_hits),
        //         handle_enemy_death.after(handle_hits),
        //     )
        //         .in_set(OnUpdate(GameState::Main)),
        // )
        // .add_system(apply_system_buffers.in_set(CustomFlush));
    }
}