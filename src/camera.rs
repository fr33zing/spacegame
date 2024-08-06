use avian3d::prelude::*;
use bevy::prelude::*;

use super::cursor::CursorPosition;
use super::player::PlayerMarker;

const CURSOR_TRACK_FACTOR: f32 = 0.15;

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
            transform: Transform::from_xyz(0., 16., 0.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
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
    cursor_position: ResMut<CursorPosition>,
    q_player_transform: Query<&Transform, (With<PlayerMarker>, Without<CameraMarker>)>,
    mut q_camera_transform: Query<&mut Transform, With<CameraMarker>>,
) {
    let player_transform = q_player_transform.single();
    let mut camera_transform = q_camera_transform.single_mut();
    let target = player_transform
        .translation
        .xz()
        .lerp(cursor_position.global.xz(), CURSOR_TRACK_FACTOR);
    camera_transform.translation.x = target.x;
    camera_transform.translation.z = target.y;
}
