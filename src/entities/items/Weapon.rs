use bevy::prelude::*;

use crate::core::{
    Bullet::{Bullet, BULLET_LIFETIME, BULLET_SPEED}, 
    Input::CursorPosition,
    graphic::Atlas::TestTextureAtlas
};

#[derive(Component)]
pub struct GunController {
    pub shoot_cooldown:f32,
    pub shoot_timer : f32
}

pub fn gun_controls(
    mut gun_query : Query<(&mut GunController, &mut Transform)>,
    cursor: Res<CursorPosition>,
    time : Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    _asset_server : Res<AssetServer>,
    handle: Res<TestTextureAtlas>,
    mut commands: Commands
) {
    for(mut gun_controller, mut transform) in gun_query.iter_mut()
    {
        gun_controller.shoot_timer -= time.delta_seconds();

        let cursor_pos = cursor.0;

        let diff = Vec2::new(cursor_pos.x - transform.translation.x, cursor_pos.y - transform.translation.y);
        let angle = diff.y.atan2(diff.x);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0.,0.,1.), angle);

        if gun_controller.shoot_timer <= 0.
        {
            if buttons.pressed(MouseButton::Right)
            {
                let mut spawn_transform = Transform::from_scale(Vec3::splat(3.0));
                spawn_transform.translation = transform.translation;
                spawn_transform.rotation = Quat::from_axis_angle(Vec3::new(0.,0.,1.), angle);
                gun_controller.shoot_timer = gun_controller.shoot_cooldown;

                commands.spawn(SpriteSheetBundle{
                    transform: spawn_transform,
                    texture: handle.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: TestTextureAtlas::get_index("bullet", &handle)
                    },
                    ..default()
                }).insert(Bullet {lifetime: BULLET_LIFETIME, speed: BULLET_SPEED, direction: diff.normalize()});
            }
        }
    }
}