use bevy::prelude::*;

#[derive(Default)]
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
) {
    // *clear_color = ClearColor(Color::srgb_u8(12, 5, 25));
    *clear_color = ClearColor(Color::srgb_u8(20, 16, 16));
    // *clear_color = ClearColor(Color::srgb_u8(15, 15, 12));
    // *clear_color = ClearColor(Color::srgb_u8(15, 14, 10));
    // *clear_color = ClearColor(Color::srgb_u8(15, 15, 15));
    commands.spawn((
        Camera2d::default(),
        Transform::default(),
    ));
}
