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
    *clear_color = ClearColor(Color::srgb_u8(20, 16, 16));
    commands.spawn((
        MainCamera,
        Camera2d::default(),
        Transform::default(),
    ));
}

#[derive(Component)]
pub struct MainCamera;