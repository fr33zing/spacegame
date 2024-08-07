use avian3d::prelude::*;
use bevy::{
    color::palettes::css::{GREEN, RED, WHITE},
    prelude::*,
};

use super::cursor::CursorPosition;

const TURN_SPEED: f32 = 8.0;
const THRUST: f32 = 256.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (look_at_cursor, thrust).chain());
        app.add_systems(
            PostUpdate,
            update_gizmos
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Component)]
pub(crate) struct PlayerMarker;

#[derive(Bundle)]
struct PlayerBundle {
    body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    scene: SceneBundle,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        PlayerBundle {
            body: RigidBody::Dynamic,
            collider: Collider::sphere(0.5),
            locked_axes: LockedAxes::new()
                .lock_translation_y()
                .lock_rotation_x()
                .lock_rotation_y()
                .lock_rotation_z(),
            scene: SceneBundle {
                scene: asset_server.load("debug_ship.glb#Scene0"),
                ..default()
            },
        },
        PlayerMarker,
    ));
}

// Variable rotation speed
fn look_at_cursor(
    time: Res<Time>,
    cursor_position: ResMut<CursorPosition>,
    mut query: Query<&mut Transform, With<PlayerMarker>>,
) {
    let mut transform = query.single_mut();
    let look_at_sphere = transform.looking_at(cursor_position.global, *transform.local_y());
    let incremental_turn_weight = TURN_SPEED * time.delta_seconds();
    let old_rotation = transform.rotation;
    transform.rotation = old_rotation.lerp(look_at_sphere.rotation, incremental_turn_weight);
}

// Constant rotation speed
// fn look_at_cursor_2(
//     time: Res<Time>,
//     cursor_position: ResMut<CursorPosition>,
//     mut query: Query<&mut Transform, With<PlayerMarker>>,
// ) {
//     let mut transform = query.single_mut();

//     let forward = (transform.rotation * Vec3::Z).xz();
//     let to_cursor = (cursor_position.global.xz() - transform.translation.xz()).normalize();
//     let forward_dot_cursor = forward.dot(to_cursor);

//     if (forward_dot_cursor - 1.0).abs() < f32::EPSILON {
//         return;
//     }

//     let right = (transform.rotation * Vec3::X).xz();
//     let right_dot_cursor = right.dot(to_cursor);
//     let rotation_sign = -f32::copysign(1.0, right_dot_cursor);
//     let max_angle = forward_dot_cursor.clamp(-1.0, 1.0).acos();
//     let rotation_angle = rotation_sign * (TURN_SPEED * time.delta_seconds()).min(max_angle);
//     transform.rotate_y(rotation_angle);
// }

fn update_gizmos(
    query: Query<(&Transform, &LinearVelocity, &Rotation), (With<RigidBody>, With<PlayerMarker>)>,
    mut gizmos: Gizmos,
) {
    for (transform, velocity, rotation) in query.iter() {
        let start = transform.translation;
        let direction = rotation * Vec3::Z;
        let end_aim = start + direction * -4.0;
        let end_velocity = start + **velocity / 2.0;
        gizmos.arrow(start, end_aim, WHITE);
        gizmos.arrow(start, end_velocity, GREEN);
    }
}

fn thrust(
    time: Res<Time<Physics>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &Rotation), (With<RigidBody>, With<PlayerMarker>)>,
) {
    for (mut velocity, rotation) in query.iter_mut() {
        velocity.clear();

        if keys.pressed(KeyCode::KeyW) {
            let direction = rotation * Vec3::Z;
            let thrust = direction * -THRUST * time.delta_seconds();

            velocity.apply_force(thrust);
        }
    }
}
