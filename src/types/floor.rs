use bevy::prelude::*;

use crate::{
    codeunits::config::Config, 
    uis::{
        sprite_kind::SpriteKind, 
        sprite_sheet::SpriteSheet,
    }
};

#[derive(Component)]
pub struct Floor {
    kind: FloorKind
}

impl Floor {
    pub fn from_kind(floor_kind: FloorKind) -> Self {
        Self { kind: floor_kind }
    }
}

pub struct FloorPlugin;
impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
    }
}

fn spawn_floor(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    config: Res<Config>,
) {
    for y in 0..15 {
        for x in 0..20 {
            let floor_kind = FloorKind::Gravel;
            commands.spawn((
                Floor::from_kind(floor_kind),
                Sprite::from_atlas_image(
                    sprite_sheet.get_handle_image(),
                    sprite_sheet.get_sprite(floor_kind.to_sprite_kind())
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

#[derive(Copy, Clone)]
pub enum FloorKind {
    Gravel,
    Road,
}

impl FloorKind {
    pub fn to_sprite_kind(&self) -> SpriteKind {
        match self {
            FloorKind::Gravel => SpriteKind::Gravel,
            FloorKind::Road => SpriteKind::Road,
        }
    }
}
