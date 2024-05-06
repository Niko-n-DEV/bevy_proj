use bevy::prelude::*;

use crate::core::{
    PlayerSystem::PlayerAttach,
    resource::graphic::Atlas::TestTextureAtlas,
    UserSystem::{
        CursorPosition,
        User
    },
    Missile::{
        Bullet, 
        BULLET_LIFETIME, 
        BULLET_SPEED
    },
    Container::Container,
    items::ItemType::{
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
    mut user_container: Query<&mut Container, With<User>>,
    cursor: Res<CursorPosition>,
    time: Res<Time>,
    _buttons: Res<ButtonInput<MouseButton>>,
    _keyboard_input: Res<ButtonInput<KeyCode>>,
    _asset_server: Res<AssetServer>,
    handle: Res<TestTextureAtlas>,
) {
    if gun_query.is_empty() && user_container.is_empty() {
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
                    for slot in container.slots.iter_mut() {
                        if slot.item_stack.item_type == ItemType::Item(Item::Ammo) {
                            if slot.item_stack.count != 0 {
                                ammo_found = true;
                                slot.item_stack.count -= 1;
                            } else {
                                slot.item_stack.item_type = ItemType::None;
                                return;
                            }
                        }
                    }
                    if !ammo_found {
                        return;
                    }

                    let mut spawn_transform = Transform::from_scale(Vec3::splat(1.0));
                    spawn_transform.translation = transform.translation;
                    spawn_transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
                    gun_controller.shoot_timer = gun_controller.shoot_cooldown;

                    commands
                        .spawn(SpriteSheetBundle {
                            transform: spawn_transform,
                            texture: handle.image.clone().unwrap(),
                            atlas: TextureAtlas {
                                layout: handle.layout.clone().unwrap(),
                                index: TestTextureAtlas::get_index("bullet", &handle),
                            },
                            ..default()
                        })
                        .insert(Name::new("Bullet"))
                        .insert(Bullet {
                            lifetime: BULLET_LIFETIME,
                            speed: BULLET_SPEED,
                            direction: diff.normalize(),
                        });
                }
            } else {
                attach.offset = Vec2::new(0., -3.);
            }
        }
    }
}
