use bevy::prelude::*;

use crate::uis::sprite_kind::SpriteKind;

#[derive(Resource, Clone)]
pub struct SpriteSheet {
    handle_image: Handle<Image>,
    handle_atlas_layout: Handle<TextureAtlasLayout>,
}
impl SpriteSheet {
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
        app.add_systems(PreStartup, load_spritesheet);
    }
}

fn load_spritesheet(
    mut commands: Commands,
    mut sprite_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>, 
) {
    let sprite_handle = asset_server.load("gfx/spritesheet24x24.png");
    let sprite_layout = TextureAtlasLayout::from_grid(
        UVec2::new(24, 24), 
        5,    // columns
        3,    // rows
        None,  // padding
        None,  // offset
    );

    commands.insert_resource(
        SpriteSheet {
            handle_image: sprite_handle,
            handle_atlas_layout: sprite_atlas_layouts.add(sprite_layout),
        }
    );
}