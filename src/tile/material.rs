use bevy::prelude::*;
use crate::tile::{
    floor::FloorKind,
    wall::WallKind,
};

#[derive(Resource, Clone)]
pub struct Material {
    pub floor: FloorKind,
    pub wall: WallKind,
}