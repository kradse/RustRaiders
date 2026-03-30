use bevy::prelude::*;

use crate::system::config::Config;

#[derive(Resource, Clone)]
pub struct Floors {
    pub dirt: SpriteKind,
    pub gravel: SpriteKind,
}
impl Default for Floors {
    fn default() -> Self {
        Self { 
            dirt: SpriteKind::FloorDirt,
            gravel: SpriteKind::FloorGravel,
         }
    }
}

#[derive(Resource, Clone)]
pub struct SpriteSheet {
    handle_image: Handle<Image>,
    handle_atlas_layout: Handle<TextureAtlasLayout>,
    pub floors: Floors,
}
impl SpriteSheet {
    // Constants
    const DEFAULT_SPRITESHEET_PATH: &'static str = "gfx/spritesheet24x24.png";
    
    pub fn get_sprite(&self, sprite_kind: SpriteKind) -> TextureAtlas 
    {
        let index = sprite_kind.get_index();

        TextureAtlas {
            layout: self.get_texture_atlas_layout(),
            index,
        }
    }
    pub fn get_handle_image(&self) -> Handle<Image>
    {
        self.handle_image.clone()
    }
    pub fn get_texture_atlas_layout(&self) -> Handle<TextureAtlasLayout>
    {
        self.handle_atlas_layout.clone()
    }
}
pub struct SpriteSheetPlugin;
impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_spritesheet);
    }
}
fn load_spritesheet(
    mut commands: Commands,
    mut sprite_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>, 
    config: Res<Config>,
) {
    let sprite_handle = asset_server.load(SpriteSheet::DEFAULT_SPRITESHEET_PATH);
    let sprite_layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.base_size, config.base_size), 
        5,    // columns
        3,    // rows
        None,  // padding
        None,  // offset
    );

    commands.insert_resource(
        SpriteSheet {
            handle_image: sprite_handle,
            handle_atlas_layout: sprite_atlas_layouts.add(sprite_layout),
            floors: Floors::default(),
        }
    );
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SpriteKind {
    FloorDirt,
    FloorGravel,
}

impl SpriteKind {
    pub fn get_index(&self) -> usize {
        match self {
            SpriteKind::FloorDirt => 0,
            SpriteKind::FloorGravel => 1,
        }
    } 
}