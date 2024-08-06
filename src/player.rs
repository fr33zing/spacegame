use avian3d::prelude::*;
use bevy::prelude::*;

use super::cursor::CursorPosition;

const TURN_SPEED: f32 = 8.0;
const THRUST: f32 = 14.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (look_at_cursor, thrust).chain());
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
    mut query_rotation: Query<&mut Transform, With<PlayerMarker>>,
) {
    let mut transform = query_rotation.single_mut();
    let look_at_sphere = transform.looking_at(cursor_position.global, *transform.local_y());
    let incremental_turn_weight = TURN_SPEED * time.delta_seconds();
    let old_rotation = transform.rotation;
    transform.rotation = old_rotation.lerp(look_at_sphere.rotation, incremental_turn_weight);
}

// Constant rotation speed
fn look_at_cursor_2(
    time: Res<Time>,
    cursor_position: ResMut<CursorPosition>,
    mut query: Query<&mut Transform, With<PlayerMarker>>,
) {
    let mut transform = query.single_mut();

    let forward = (transform.rotation * Vec3::Z).xz();
    let to_cursor = (cursor_position.global.xz() - transform.translation.xz()).normalize();
    let forward_dot_cursor = forward.dot(to_cursor);

    if (forward_dot_cursor - 1.0).abs() < f32::EPSILON {
        return;
    }

    let right = (transform.rotation * Vec3::X).xz();
    let right_dot_cursor = right.dot(to_cursor);
    let rotation_sign = -f32::copysign(1.0, right_dot_cursor);
    let max_angle = forward_dot_cursor.clamp(-1.0, 1.0).acos();
    let rotation_angle = rotation_sign * (TURN_SPEED * time.delta_seconds()).min(max_angle);
    transform.rotate_y(rotation_angle);
}

fn thrust(time: Res<Time>, mut query: Query<(&mut LinearVelocity, &Rotation), With<RigidBody>>) {
    for (mut velocity, rotation) in query.iter_mut() {
        let direction = rotation * Vec3::Z;
        let thrust = direction * -THRUST * time.delta_seconds();
        let damping = 1.0 - 0.99999999 * time.delta_seconds(); // TODO Clean this

        velocity.x *= damping;
        velocity.z *= damping;

        velocity.x += thrust.x;
        velocity.z += thrust.z;
        break;
    }
}
