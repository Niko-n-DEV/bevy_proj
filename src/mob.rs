use bevy::prelude::*;

use crate::Player;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_mob_parent)
            .add_systems(Update, (spawn_mob, mob_lifetime))
            .register_type::<Mob>();
    }
}

#[derive(Default)]
#[reflect(Component)]
pub struct Mob {
    pub lifetime: Timer,
}

pub struct MobParent;

fn spawn_mob_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), MobParent, Name::new("Mob Parent")));
}

fn spawn_mob(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<MobParent>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();

    let texture = asset_server.load("mob.png");

        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: *player_transform,
                    ..default()
                },
                Mob {
                    lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                },
                Name::new("Mob"),
            ));
        });
}

fn mob_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut mobs: Query<(Entity, &mut Mob)>,
    parent: Query<Entity, With<MobParent>>
) {
    let parent = parent.single();

    for (mob_entity, mut mob) in &mut mobs {
        mob.lifetime.tick(time.delta());

        if mob.lifetime.finished() {
            commands.entity(parent).remove_children(&[mob_entity]);
            commands.entity(mob_entity).despawn();
        }
    }
}