use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings}
    },
};

mod codeunits;
use codeunits::{
    camera::CameraPlugin,
    config::ConfigPlugin,
    input::InputPlugin,
};

mod types;
use types::{
    floor::FloorPlugin,
    wall::WallPlugin,
};

mod uis;
use uis::{
    sprite_sheet::SpriteSheetPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rust Raiders".into(),
                    // resolution: (640, 360).into(), // Native resolution (16:9)
                    resolution: (1280, 720).into(), // 720p (16:9)
                    // resolution: (1920, 1080).into(), // 1080p (16:9)
                    // resolution: (2560, 1440).into(), // 1440p (16:9)
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((
            CameraPlugin, ConfigPlugin, InputPlugin,
            FloorPlugin, WallPlugin, // EnemyPlugin,
            SpriteSheetPlugin,
        ))
        .run();
}
