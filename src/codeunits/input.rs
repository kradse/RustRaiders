use bevy::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera);
    }
}

fn move_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut transform) = query.single_mut() else 
        { return; };

    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        transform.translation.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        transform.translation.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        transform.translation.y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        transform.translation.x -= 1.;
    }
}