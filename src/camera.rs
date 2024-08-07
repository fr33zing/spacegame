use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::player::PlayerMarker;

const CURSOR_TRACK_MAX_DISTANCE: f32 = 4.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            PostUpdate,
            update
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Component)]
pub struct CameraMarker;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 16., 0.).looking_at(Vec3::new(0., 0., 0.), -Vec3::Z),
            ..default()
        },
        CameraMarker,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight { ..default() },
        transform: Transform::from_xyz(0., 10., 0.),
        ..default()
    });
}

fn update(
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_player_transform: Query<
        &Transform,
        (With<RigidBody>, With<PlayerMarker>, Without<CameraMarker>),
    >,
    mut query_camera_transform: Query<&mut Transform, With<CameraMarker>>,
) {
    let cursor_offset = 'co: {
        let window = query_window.single();
        let Some(cursor_position) = window.cursor_position() else {
            break 'co Vec2::ZERO;
        };
        let window_size = window.size();
        let cursor_offset =
            (cursor_position / window.size() - Vec2::new(0.5, 0.5)) * CURSOR_TRACK_MAX_DISTANCE;
        let aspect_ratio_adjustment = if window_size.x > window_size.y {
            Vec2::new(window_size.x / window_size.y, 1.0)
        } else {
            Vec2::new(1.0, window_size.y / window_size.x)
        };

        cursor_offset * aspect_ratio_adjustment
    };

    let player_transform = query_player_transform.single();
    let mut camera_transform = query_camera_transform.single_mut();
    let target = player_transform.translation.xz() + cursor_offset;
    camera_transform.translation.x = target.x;
    camera_transform.translation.z = target.y;
}
