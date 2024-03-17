use bevy::{prelude::*, window::PrimaryWindow};

use crate::core::{bullet::{Bullet, BULLET_LIFETIME, BULLET_SPEED}, Input::OffsetedCursorPosition};

#[derive(Component)]
pub struct GunController {
    pub shoot_cooldown:f32,
    pub shoot_timer : f32
}

pub fn gun_controls(
    mut cursor_res: ResMut<OffsetedCursorPosition>,
    mut gun_query : Query<(&mut GunController, &mut Transform)>,
    mut cursor: EventReader<CursorMoved>, primary_query: Query<&Window, With<PrimaryWindow>>,
    time : Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    asset_server : Res<AssetServer>,
    mut commands: Commands
) {
    for(mut gun_controller, mut transform) in gun_query.iter_mut()
    {
        
        gun_controller.shoot_timer -= time.delta_seconds();
        
        let Ok(primary) = primary_query.get_single() else
        {
            return;
        };
        let mut cursor_position = cursor.read().last().map(|event| event.position).unwrap_or_else(|| Vec2::new(cursor_res.x + primary.width() / 2., cursor_res.y + primary.height() / 2.));
        
        cursor_position.x -= primary.width()/2.;
        cursor_position.y -= primary.height()/2.;

        cursor_res.x = cursor_position.x;
        cursor_res.y = cursor_position.y;

        let diff = Vec2::new(cursor_position.x - transform.translation.x, transform.translation.y - cursor_position.y);
        let angle = (-diff.y).atan2(diff.x);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0.,0.,1.), angle);

        if gun_controller.shoot_timer <= 0.
        {
            if buttons.pressed(MouseButton::Left)
            {
                let mut spawn_transform = Transform::from_scale(Vec3::splat(3.0));
                spawn_transform.translation = transform.translation;
                spawn_transform.rotation = Quat::from_axis_angle(Vec3::new(0.,0.,1.), angle);
                gun_controller.shoot_timer = gun_controller.shoot_cooldown;

                commands.spawn(SpriteBundle{
                    transform: spawn_transform,
                    texture: asset_server.load("bullet.png"),
                    ..default()
                }).insert(Bullet {lifetime: BULLET_LIFETIME, speed: BULLET_SPEED, direction: diff.normalize()});
            }
        }
    }
}