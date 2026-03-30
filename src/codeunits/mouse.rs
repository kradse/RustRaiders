use bevy::prelude::*;

use crate::{
    codeunits::config::Config,
    types::{
        toolbox::{ToolKind, Toolbox},
        wall::{Wall, GridPosition},
    },
};

pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_mouse_click);
    }
}

fn handle_mouse_click(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    toolbox: Query<&Toolbox>,
    wall_query: Query<(Entity, &GridPosition), With<Wall>>,
    config: Res<Config>,
) {
    // Only handle left mouse button clicks
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // Check if selection tool is active
    let Ok(active_tool) = toolbox.single() else { return; };
    if active_tool.active != ToolKind::Selection {
        return;
    }

    // Get cursor world position
    let Ok(window) = window_query.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    
    let Ok((camera, camera_transform)) = camera_query.single() else { return; };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };
    
    // Calculate grid position
    let tile_size = config.scaled_size();
    let grid_x = (world_pos.x / tile_size).round() as i32;
    let grid_y = (world_pos.y / tile_size).round() as i32;
    
    // Find and despawn wall at this grid position
    for (entity, grid_pos) in wall_query.iter() {
        if grid_pos.x == grid_x && grid_pos.y == grid_y {
            commands.entity(entity).despawn();
            break;
        }
    }
}