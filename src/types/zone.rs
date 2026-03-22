use bevy::prelude::*;

use crate::{
    codeunits::config::Config, 
    types::tile::Tile, 
    uis::{
        sprite_kind::SpriteKind, 
        sprite_sheet::SpriteSheet,
    }
};

#[derive(Component)]
pub struct Zone;

pub struct ZonePlugin;
impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
        app.add_systems(Startup, spawn_walls);
    }
}

fn spawn_floor(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    config: Res<Config>,
) {
    for y in 0..15 {
        for x in 0..20 {
            commands.spawn((
                Zone,
                Tile,
                Sprite::from_atlas_image(
                    sprite_sheet.get_handle_image(), 
                    sprite_sheet.get_sprite(SpriteKind::Red)
                ),
                Transform::from_xyz(
                    config.scaled_size() * x as f32,
                    -(config.scaled_size() * y as f32),
                    100.
                ).with_scale(Vec3::new(
                    config.scale, 
                    config.scale, 
                    0.)
                ),
            ));
        };
    };
}

fn spawn_walls(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    config: Res<Config>,
) {
    for y in 0..8 {
        for x in 0..10 {
            commands.spawn((
                Zone,
                Tile,
                Sprite::from_atlas_image(
                    sprite_sheet.get_handle_image(), 
                    sprite_sheet.get_sprite(SpriteKind::Green)
                ),
                Transform::from_xyz(
                    config.scaled_size() * x as f32,
                    -(config.scaled_size() * y as f32),
                    200.
                ).with_scale(Vec3::new(
                    config.scale, 
                    config.scale, 
                    0.)
                ),
            ));
        };
    };
}