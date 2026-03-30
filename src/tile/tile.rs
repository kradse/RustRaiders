use bevy::prelude::*;
use bevy::math::IVec2;

use crate::spritesheet::spritesheet::SpriteSheet;
use crate::system::config::Config;
use crate::tile::floor::{Floor, FloorKind};

#[derive(Component)]
struct Tile {
    grid_position: IVec2,
}
impl Tile {
    pub fn from_xy(x: i32, y: i32) -> Self {
        Self { 
            grid_position: IVec2 {x, y} 
        }
    }
}

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_tile);
    }
}

fn spawn_tile(
    mut commands: Commands,
    config: Res<Config>,
    sprite_sheet: Res<SpriteSheet>,
) {
    
    for y in 0..2 {
        for x in 0..3 {
            commands.spawn((
                Tile::from_xy(x, y),
                Floor::from_kind(FloorKind::Dirt),
                Sprite::from_atlas_image(
                    sprite_sheet.get_handle_image(),
                    sprite_sheet.get_sprite(FloorKind::sprite_kind(FloorKind::Dirt)),
                ),
                Transform::from_xyz(
                    x_pos(&config, &x), 
                    y_pos(&config, &y), 
                    100.
                ).with_scale(Vec3::new(
                    config.scale as f32, 
                    config.scale as f32, 
                    0.)
                ),
            ));
        }
    }
}

fn x_pos(config: &Res<Config>, x: &i32) -> f32 {
    config.scaled_size_f32() * *x as f32
}
fn y_pos(config: &Res<Config>, y: &i32) -> f32 {
    -(config.scaled_size_f32() * *y as f32)
}