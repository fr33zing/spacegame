use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{Cursor, PresentMode},
};

mod camera;
mod cursor;
mod player;

use camera::CameraPlugin;
use cursor::{CursorPlugin, PhysicsCursorPlugin};
use player::PlayerPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins // TODO Clean this
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    cursor: Cursor {
                        visible: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
        PhysicsPlugins::default().build().add(PhysicsCursorPlugin),
        PhysicsDebugPlugin::default(),
        CursorPlugin,
        CameraPlugin,
        PlayerPlugin,
    ));
    //app.insert_resource(Time::new_with(Physics::variable(1.0 / 600.0)));
    app.run();
}
