use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct NFTacticsAssets {
    // ! AUDIO
    #[asset(path = "audio/2024-07-03 nftactics track 0 2024-07-03 1732.ogg")]
    pub audio_nftactics_track_0: Handle<AudioSource>,

    // ! TEXTURES
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 88, rows = 45))]
    pub spritesheet_the_roguelike_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "gfx/The Roguelike 1-13-3 Alpha.png")]
    pub spritesheet_the_roguelike_image: Handle<Image>,
}
