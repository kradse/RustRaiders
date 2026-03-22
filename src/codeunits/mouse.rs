use bevy::prelude::*;

use crate::{
    codeunits::config::Config, 
    types::toolbox::{
        ToolKind, Toolbox
    }, 
};

pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_selection_box);
        app.add_systems(Update, move_selection_box);
    }
}

fn move_selection_box(
    mut query: Query<(&mut Sprite, &mut Transform), With<SelectionBox>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    toolbox: Query<&Toolbox>,
    config: Res<Config>,
) {
    let Ok(active_tool) = toolbox.single() else
        { return; };

    let Ok((mut sprite, mut transform)) = query.single_mut() else
        { return; };

    if active_tool.active == ToolKind::Selection {
        sprite.color = Color::srgba(0.0, 1.0, 0.0, 0.25);
    } else {
        sprite.color = Color::srgba(0.0, 1.0, 0.0, 0.0);
    }

    // Get primary window
    let Ok(window) = window_query.single() else 
        { return; };
    
    // Get cursor position (returns None if cursor is outside window)
    let Some(cursor_pos) = window.cursor_position() else 
        { return; };
    
    // Get camera and its global transform
    let Ok((camera, camera_transform)) = camera_query.single() else 
        { return; };
    
    // Convert viewport coordinates to world coordinates
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else 
        { return; };
    
    // Calculate grid position
    let tile_size = config.scaled_size();
    let grid_x = (world_pos.x / tile_size).round();
    let grid_y = (world_pos.y / tile_size).round();
    
    // Snap to grid center
    let snap_x = grid_x * tile_size;
    let snap_y = grid_y * tile_size;
    
    // Update transform (keep z-position unchanged)
    transform.translation.x = snap_x;
    transform.translation.y = snap_y;
}

#[derive(Component)]
struct SelectionBox;

fn spawn_selection_box(
    mut commands: Commands,
    config: Res<Config>,
) {
    commands.spawn((
        SelectionBox,
        Sprite {
            color: Color::srgba(0.0, 1.0, 0.0, 0.25),
            custom_size: Some(Vec2::new(
                Config::BASE_SIZE as f32,
                Config::BASE_SIZE as f32,
            )),
            ..default()
        },
        Transform::from_xyz(
            config.scaled_size() * 2 as f32,
            -(config.scaled_size() * 3 as f32),
            300.
        ).with_scale(Vec3::new(
            config.scale,
            config.scale,
            0.)
        ),
    ));
}