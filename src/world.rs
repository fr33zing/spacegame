use avian3d::prelude::*;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        RigidBody::Static,
        Collider::sphere(4.0),
        SceneBundle {
            scene: asset_server.load("sun.glb#Scene0"),
            ..default()
        },
    ));
}
