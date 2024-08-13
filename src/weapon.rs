use std::time::{Duration, Instant};

use avian3d::prelude::*;
use bevy::prelude::*;

//
// Plugin
//

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fire, remove_dead_bullets));
        app.add_systems(PostProcessCollisions, prevent_bullet_owner_collision);
    }
}

//
// Components
//

#[derive(Component)]
pub struct Weapon {
    firing: bool,
    last_fired: Instant,
    cooldown: Duration,
    muzzle_velocity: f32,
}

impl Weapon {
    pub fn with_rounds_per_minute(rpm: f32) -> Self {
        let cooldown = Duration::from_secs_f32(60.0 / rpm);
        Self {
            last_fired: Instant::now() - cooldown,
            firing: false,
            cooldown,
            muzzle_velocity: 20.0,
        }
    }

    pub fn start_firing(&mut self) {
        if self.firing {
            return;
        }
        self.firing = true;
        let no_cooldown = Instant::now() - self.cooldown;
        if self.last_fired < no_cooldown {
            self.last_fired = no_cooldown;
        }
    }

    pub fn stop_firing(&mut self) {
        self.firing = false;
    }
}

#[derive(Component)]
pub(crate) struct BulletMarker;

#[derive(Component, Deref)]
pub(crate) struct BulletOwner(Entity);

#[derive(Component, Deref)]
pub(crate) struct BulletDeathTime(Instant);

#[derive(Bundle)]
struct BulletBundle {
    body: RigidBody,
    mass: Mass,
    collider: Collider,
    locked_axes: LockedAxes,
    scene: SceneBundle,
    owner: BulletOwner,
    death_time: BulletDeathTime,
}

//
// Systems
//

fn fire(
    mut query: Query<(Entity, &Transform, &LinearVelocity, &Mass, &mut Weapon), With<RigidBody>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (owner, owner_transform, owner_velocity, owner_mass, mut weapon) in &mut query {
        if !weapon.firing {
            continue;
        }

        let since_last_fire = Instant::now() - weapon.last_fired;
        let fire_times =
            (since_last_fire.as_secs_f64() / weapon.cooldown.as_secs_f64()).floor() as u32;

        if fire_times > 0 {
            weapon.last_fired = Instant::now();
        }

        let fire_direction = owner_transform.rotation * -Vec3::Z;

        for _ in 0..fire_times {
            let mass: f32 = 0.1;
            let muzzle_velocity = fire_direction * (weapon.muzzle_velocity / mass);
            let velocity = muzzle_velocity + **owner_velocity * (**owner_mass / mass);

            commands.spawn((
                BulletBundle {
                    body: RigidBody::Dynamic,
                    mass: Mass(mass),
                    collider: Collider::sphere(0.1),
                    locked_axes: LockedAxes::new()
                        .lock_translation_y()
                        .lock_rotation_x()
                        .lock_rotation_y()
                        .lock_rotation_z(),
                    scene: SceneBundle {
                        scene: asset_server.load("bullet.glb#Scene0"),
                        transform: owner_transform.clone(),
                        ..default()
                    },
                    owner: BulletOwner(owner),
                    death_time: BulletDeathTime(Instant::now() + Duration::from_secs(10)),
                },
                ExternalForce::new(velocity).with_persistence(false),
                BulletMarker,
            ));
        }
    }
}

fn remove_dead_bullets(
    query: Query<(Entity, &BulletDeathTime), With<BulletMarker>>,
    mut commands: Commands,
) {
    let now = Instant::now();
    query.iter().for_each(|(bullet, death_time)| {
        if **death_time <= now {
            commands.entity(bullet).despawn();
        }
    });
}

fn prevent_bullet_owner_collision(
    mut collisions: ResMut<Collisions>,
    query: Query<(Entity, &BulletOwner), With<BulletMarker>>,
) {
    for (bullet, owner) in &query {
        collisions.remove_collision_pair(bullet, **owner);
    }
}
