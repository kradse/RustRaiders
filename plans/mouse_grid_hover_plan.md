# Mouse Grid Hover Implementation Plan

## Overview
Implement mouse cursor tracking to snap the SelectionBox to the grid tile the cursor is hovering over in the [`move_selection_box`](src/codeunits/mouse.rs:15) system.

## Current State Analysis

### Existing Code
- **SelectionBox Component**: Already defined at line 24-25
- **spawn_selection_box**: Creates a green semi-transparent sprite (lines 27-51)
- **move_selection_box**: Empty implementation that needs logic (lines 15-22)

### Grid System Understanding
From [`floor.rs`](src/types/floor.rs:29):
```rust
Transform::from_xyz(
    config.scaled_size() * x as f32,
    -(config.scaled_size() * y as f32),
    100.
)
```

**Grid Characteristics:**
- Tile size: `config.scaled_size()` (24 * scale)
- X-axis: Positive right, tiles at [0, scaled_size, 2*scaled_size, ...]
- Y-axis: Negative down, tiles at [0, -scaled_size, -2*scaled_size, ...]
- Grid centers are at multiples of `scaled_size()`

## Technical Approach

### Required Queries and Resources

Add to [`move_selection_box`](src/codeunits/mouse.rs:15) function signature:
```rust
fn move_selection_box(
    mut query: Query<(&mut Sprite, &mut Transform), With<SelectionBox>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    config: Res<Config>,
)
```

### Implementation Steps

#### 1. Get Cursor Position
```rust
let Ok(window) = window_query.single() else { return; };
let Some(cursor_pos) = window.cursor_position() else { return; };
```

**cursor_pos** is in viewport coordinates (pixels from bottom-left)

#### 2. Get Camera Transform
```rust
let Ok((camera, camera_transform)) = camera_query.single() else { return; };
```

#### 3. Convert Viewport to World Coordinates
```rust
let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) 
    else { return; };
```

**world_pos** is now in world space (Bevy coordinates)

#### 4. Calculate Grid Position
```rust
let tile_size = config.scaled_size();
let grid_x = (world_pos.x / tile_size).floor();
let grid_y = (world_pos.y / tile_size).floor();
```

**Why floor()?** 
- For positive X: floor(1.7) = 1.0 → tile 1
- For negative Y: floor(-0.3) = -1.0 → tile -1
- This correctly snaps to the grid cell containing the cursor

#### 5. Snap to Grid Center
```rust
let snap_x = grid_x * tile_size;
let snap_y = grid_y * tile_size;
```

**Example:**
- cursor at world (37.2, -51.8), tile_size = 24
- grid_x = 37.2/24 = 1.55 → floor = 1.0
- grid_y = -51.8/24 = -2.16 → floor = -3.0
- snap_x = 1.0 * 24 = 24.0
- snap_y = -3.0 * 24 = -72.0
- SelectionBox moves to (24, -72)

#### 6. Update Transform
```rust
transform.translation.x = snap_x;
transform.translation.y = snap_y;
// Keep z and scale unchanged
```

## Complete Implementation

### File: [`src/codeunits/mouse.rs`](src/codeunits/mouse.rs:15)

```rust
fn move_selection_box(
    mut query: Query<(&mut Sprite, &mut Transform), With<SelectionBox>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    config: Res<Config>,
) {
    let Ok((mut sprite, mut transform)) = query.single_mut() else 
        { return; };

    // Get primary window
    let Ok(window) = window_query.single() else { return; };
    
    // Get cursor position (returns None if cursor is outside window)
    let Some(cursor_pos) = window.cursor_position() else { return; };
    
    // Get camera and its global transform
    let Ok((camera, camera_transform)) = camera_query.single() else { return; };
    
    // Convert viewport coordinates to world coordinates
    let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) 
        else { return; };
    
    // Calculate grid position
    let tile_size = config.scaled_size();
    let grid_x = (world_pos.x / tile_size).floor();
    let grid_y = (world_pos.y / tile_size).floor();
    
    // Snap to grid center
    let snap_x = grid_x * tile_size;
    let snap_y = grid_y * tile_size;
    
    // Update transform (keep z-position unchanged)
    transform.translation.x = snap_x;
    transform.translation.y = snap_y;
}
```

## Behavior

### When Cursor is Inside Window
- SelectionBox follows cursor smoothly
- Snaps to grid tiles matching floor tile positions
- Visible at all times (existing spawn has no Visibility component)

### When Cursor is Outside Window
- `window.cursor_position()` returns None
- Function returns early
- SelectionBox stays at last known position

## Z-Ordering

Current z-positions:
- Floor tiles: z = 100
- Walls: z = 200
- **SelectionBox: z = 200** (same as walls)

SelectionBox is spawned at z=200, ensuring it's visible above floor tiles.

## Testing Checklist

- [ ] SelectionBox snaps to correct grid positions
- [ ] SelectionBox follows mouse smoothly
- [ ] SelectionBox works when camera moves (WASD/Arrow keys)
- [ ] SelectionBox stays in last position when cursor leaves window
- [ ] Grid snapping matches floor tile positions
- [ ] No visual lag or stuttering
- [ ] SelectionBox visible over floor tiles

## Edge Cases

### Negative Coordinates
- Grid properly handles negative Y values (floor tiles go negative)
- `floor()` function handles negatives correctly

### Camera Movement
- Uses `viewport_to_world_2d` which accounts for camera position
- Grid calculation is in world space, not screen space
- Should work seamlessly as camera moves

### Multiple Windows
- Uses `single()` query, assumes one primary window
- Appropriate for single-window applications

## Future Enhancements

### Optional: Different Behavior When Cursor Outside
```rust
let Some(cursor_pos) = window.cursor_position() else {
    // Hide selection box when cursor leaves window
    sprite.color.set_alpha(0.0);
    return;
};
// Make visible when cursor returns
sprite.color.set_alpha(0.25);
```

### Optional: Grid Coordinate Display
Could add debug text showing current grid coordinates:
```rust
println!("Grid: ({}, {})", grid_x as i32, grid_y as i32);
```

### Optional: Limit to Grid Bounds
If you want to restrict to the floor grid (20x15):
```rust
let grid_x = (world_pos.x / tile_size).floor().clamp(0.0, 19.0);
let grid_y = (world_pos.y / tile_size).floor().clamp(-14.0, 0.0);
```

## Dependencies

No new dependencies required - all functionality uses Bevy core:
- `Window::cursor_position()` - Built-in
- `Camera::viewport_to_world_2d()` - Built-in
- `GlobalTransform` - Built-in

## Related Files

- [`src/codeunits/config.rs`](src/codeunits/config.rs:1) - Config resource with scaled_size()
- [`src/types/floor.rs`](src/types/floor.rs:29) - Reference for grid positioning
- [`src/codeunits/camera.rs`](src/codeunits/camera.rs:1) - Camera setup
