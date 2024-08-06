use avian3d::prelude::*;
use bevy::{math::VectorSpace, prelude::*, window::PrimaryWindow};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};

use super::camera::CameraMarker;

pub struct PhysicsCursorPlugin;
pub struct CursorPlugin;

#[derive(Component)]
struct GameCursor;

impl Plugin for PhysicsCursorPlugin {
    fn build(&self, app: &mut App) {
        let physics_schedule = app
            .get_schedule_mut(PhysicsSchedule)
            .expect("add PhysicsSchedule first");

        physics_schedule.add_systems(
            update_world_cursor
                .after(PhysicsStepSet::NarrowPhase)
                .after(PhysicsStepSet::Solver),
        );
    }
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>();
        app.add_plugins(InfiniteGridPlugin);
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_screen_cursor);
    }
}

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub global: Vec3,
    pub local: Vec2,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(InfiniteGridBundle::default());
    commands.spawn((
        ImageBundle {
            image: asset_server.load("cursor.png").into(),
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex::Global(15),
            ..default()
        },
        GameCursor {},
    ));
}

fn update_screen_cursor(
    query_window: Query<&Window, With<PrimaryWindow>>,
    mut query_cursor: Query<&mut Style, With<GameCursor>>,
) {
    let window: &Window = query_window.single();
    if let Some(position) = window.cursor_position() {
        let mut img_style = query_cursor.single_mut();
        img_style.left = Val::Px(position.x - 16.0);
        img_style.top = Val::Px(position.y - 16.0);
    }
}

fn update_world_cursor(
    mut cursor_position_res: ResMut<CursorPosition>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<CameraMarker>>,
    query_plane: Query<&GlobalTransform, With<InfiniteGrid>>,
) {
    // Based on "Convert cursor to world coordinates" from Unofficial Bevy Cheat Book
    // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    let (camera, camera_transform) = query_camera.single();
    let ground_transform = query_plane.single();
    let window = query_window.single();

    let Some(cursor_position) = window.cursor_position() else {
        return; // Cursor is outside window
    };

    let plane_origin = ground_transform.translation();
    let plane = InfinitePlane3d::new(ground_transform.up());

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
        return; // Ray does not intersect ground
    };

    let global_cursor = ray.get_point(distance);
    let inverse_transform_matrix = ground_transform.compute_matrix().inverse();
    let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);

    cursor_position_res.global = global_cursor;
    cursor_position_res.global.y = 0.0; // TODO Why is this necessary?
    cursor_position_res.local = local_cursor.xz();
}
