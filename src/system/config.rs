use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub base_size: u32,
    pub scale: u32,
}

impl Config {
	// Constants
    const DEFAULT_CONFIG_PATH: &'static str = "assets/config/config.json";
    
	// Constructors
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&json)?;
        Ok(config)
    }
    
	// Public functions
    pub fn scaled_size(&self) -> u32 {
        self.base_size * self.scale
    }
    pub fn scaled_size_f32(&self) -> f32 {
        self.scaled_size() as f32
    }
	// Private functions
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_size: 24,
            scale: 1,
        }
    }
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_config);
    }
}

fn load_config(mut commands: Commands) {
    let Ok(config) = Config::from_file(Config::DEFAULT_CONFIG_PATH) else {
        return commands.insert_resource(Config::default());
    };
    
    commands.insert_resource(config);
}