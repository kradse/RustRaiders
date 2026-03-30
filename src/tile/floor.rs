use bevy::prelude::*;
use crate::spritesheet::spritesheet::SpriteKind;

#[derive(Component)]
pub struct Floor {
    kind: FloorKind,
}

impl Floor {
    // Constants
	// Constructors
    pub fn from_kind(kind: FloorKind) -> Self {
        Self { kind }
    }
	// Public functions
	// Private functions
}

#[derive(Copy, Clone)]
pub enum FloorKind {
    Dirt,
    Gravel,
}
impl FloorKind {
    pub fn sprite_kind(kind: FloorKind) -> SpriteKind {
        match kind {
            FloorKind::Dirt => SpriteKind::FloorDirt,
            FloorKind::Gravel => SpriteKind::FloorGravel,
        }
    }
}