use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings}
    },
};

mod camera;
use camera::camera::CameraPlugin;

mod system;
use system::config::ConfigPlugin;

mod spritesheet;
use spritesheet::spritesheet::SpriteSheetPlugin;

mod tile;
use tile::tile::TilePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rust Raiders".into(),
                    resolution: (480, 360).into(), // Native resolution (4:3)
                    // resolution: (1280, 720).into(), // 720p (16:9)
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
            TilePlugin,
            ConfigPlugin,
            CameraPlugin,
            SpriteSheetPlugin,
        ))
        .run();
}
