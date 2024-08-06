use avian3d::prelude::*;
use bevy::{prelude::*, window::PresentMode};

mod camera;
mod cursor;
mod player;

use camera::CameraPlugin;
use cursor::CursorPlugin;
use player::PlayerPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins // TODO Clean this
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default(),
        CameraPlugin,
        CursorPlugin,
        PlayerPlugin,
    ));
    app.run();
}
