use bevy::prelude::*;

use crate::{
    codeunits::config::Config,
    uis::{
        sprite_kind::SpriteKind,
        sprite_sheet::SpriteSheet,
    }
};

#[derive(Component)]
pub struct Wall {
    kind: WallKind
}

#[derive(Component, Clone, Copy)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl Wall {
    pub fn from_kind(wall_kind: WallKind) -> Self {
        Self { kind: wall_kind }
    }
}

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_wall);
    }
}

fn spawn_wall(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    config: Res<Config>,
) {
    for y in 0..7 {
        for x in 0..10 {
            let wall_kind = WallKind::Rock;
            commands.spawn((
                Wall::from_kind(wall_kind),
                GridPosition { x: x as i32, y: -(y as i32) },
                Sprite::from_atlas_image(
                    sprite_sheet.get_handle_image(),
                    sprite_sheet.get_sprite(wall_kind.to_sprite_kind())
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




    // commands.spawn((
    //     Sprite {
    //         color: Color::srgba(0.0, 1.0, 0.0, 0.15), // Gul, 30% opacity
    //         custom_size: Some(Vec2::new(
    //             Config::BASE_SIZE as f32,
    //             Config::BASE_SIZE as f32,
    //         )),
    //         ..default()
    //     },
    //     Transform::from_xyz(
    //         config.scaled_size() * 11 as f32,
    //         -(config.scaled_size() * 2 as f32),
    //         300.
    //     ).with_scale(Vec3::new(
    //         config.scale,
    //         config.scale,
    //         0.)
    //     ),
    // ));



}

#[derive(Copy, Clone)]
pub enum WallKind {
    Rock,
}

impl WallKind {
    pub fn to_sprite_kind(&self) -> SpriteKind {
        match self {
            WallKind::Rock => SpriteKind::WallBrown,
        }
    }
}
