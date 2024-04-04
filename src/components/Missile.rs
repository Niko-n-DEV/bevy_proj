use bevy::prelude::*;
/* Потом переделаю под снаряды */
use crate::core::{player::PlayerEntity::PlayerEntity, Entity::EntityBase};

pub const BULLET_LIFETIME: f32 = 10.0;
pub const BULLET_SPEED: f32 = 3000.;

#[derive(Component)]
pub struct Bullet {
    pub lifetime: f32,
    pub speed: f32,
    pub direction: Vec2,
}

pub fn update_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(&mut Bullet, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    if bullet_query.is_empty() {
        return;
    }
    
    for (mut bullet, mut transform, entity) in bullet_query.iter_mut() {
        bullet.lifetime -= time.delta_seconds();
        let moving = bullet.speed * bullet.direction * time.delta_seconds();
        transform.translation += Vec3::new(moving.x, moving.y, 0.);

        if bullet.lifetime <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

pub struct BulletInfo {
    pub translation: Vec3,
    pub entity: Entity,
}

pub fn update_bullet_hits(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity), (With<Bullet>, Without<EntityBase>)>,
    mut enemy_query: Query<(&mut EntityBase, &mut Transform), (Without<Bullet>, Without<PlayerEntity>)>,
) {
    if bullet_query.is_empty() {
        return;
    }

    let mut bullet_list = Vec::new();
    for (transform, entity) in bullet_query.iter() {
        bullet_list.push(BulletInfo {
            translation: Vec3::new(transform.translation.x, transform.translation.y, 0.),
            entity: entity,
        });
    }
    let mut bullet_len = bullet_list.len();
    for (mut enemy, transform) in enemy_query.iter_mut() {
        let mut i: i32 = 0;
        while i < bullet_len as i32 {
            if Vec3::distance(
                bullet_list[i as usize].translation,
                Vec3::new(transform.translation.x, transform.translation.y, 0.),
            ) <= 36. {
                enemy.health.0 -= 1.;

                commands.entity(bullet_list[i as usize].entity).despawn();

                bullet_list.remove(i as usize);

                i -= 1;

                bullet_len -= 1;
            }
            i += 1;
        }
    }
}
