use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{Cursor, PresentMode, WindowMode},
};

mod camera;
mod cursor;
mod player;
mod weapon;

use camera::CameraPlugin;
use cursor::CursorPlugin;
use player::PlayerPlugin;
use weapon::WeaponPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins // TODO Clean this
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    present_mode: PresentMode::AutoNoVsync,
                    cursor: Cursor {
                        visible: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
        PhysicsPlugins::default().build(),
        PhysicsDebugPlugin::default(),
        CursorPlugin,
        CameraPlugin,
        PlayerPlugin,
        WeaponPlugin,
    ));
    app.run();
}
