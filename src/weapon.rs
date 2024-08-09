use std::time::{Duration, Instant};

use avian3d::prelude::*;
use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, fire);
    }
}

#[derive(Component)]
pub struct Weapon {
    firing: bool,
    last_fired: Instant,
    cooldown: Duration,
}

impl Weapon {
    pub fn with_rounds_per_minute(rpm: f32) -> Self {
        let cooldown = Duration::from_secs_f32(60.0 / rpm);
        Self {
            last_fired: Instant::now() - cooldown,
            firing: false,
            cooldown,
        }
    }

    pub fn start_firing(&mut self) {
        self.firing = true;
        self.last_fired = Instant::now() - self.cooldown;
    }

    pub fn stop_firing(&mut self) {
        self.firing = false;
    }
}

fn fire(time: Res<Time>, mut query: Query<(&Rotation, &mut Weapon), With<RigidBody>>) {
    for (transform, mut weapon) in &mut query {
        if !weapon.firing {
            continue;
        }

        let since_last_fire = Instant::now() - weapon.last_fired;
        let fire_times = since_last_fire.as_micros() / weapon.cooldown.as_micros();

        if fire_times > 0 {
            weapon.last_fired = Instant::now();
        }

        for _ in 0..fire_times {
            //
        }
    }
}
