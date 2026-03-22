use bevy::prelude::*;

#[derive(Resource)]
pub struct Config {
    pub scale: f32,
}

impl Config {
    // Constants
    pub const BASE_SIZE: usize = 24;

    // Public functions
    pub fn scaled_size(&self) -> f32
    {
        self.scale * Self::BASE_SIZE as f32
    }

    // Constructors
    pub const fn new(scale: f32) -> Self {
        Self { scale }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { scale: 1.0 }
    }
}

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config::new(2.));
    }
}
