use assets::NFTacticsAssets;
use bevy::{audio::Volume, prelude::*};
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use bevy_ecs_tilemap::prelude::*;
use ui::{main_menu::main_menu_plugin, splash::splash_plugin};

mod assets;
mod bevy_ecs_tilemap_helper_tiled;
mod resources;
mod ui;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Splash,
    MainMenu,
    Game,
}

// static CONFIG: &str = r#"
//     name = "NFTactics"
//     version = "0.1.0"
//     author = "Mike Hukiewitz, David Alexander Pfeiffer"
//     description = "Next-gen web3 auto battler"
//     [settings]
//     resolution = [256, 144]
//     [solana]
//     http-rpc-url = "http://127.0.0.1:8899"
//     ws-rpc-url = "ws://127.0.0.1:8900"
// "#;

fn init_map() -> tiled::Map {
    let mut loader = tiled::Loader::new();

    loader.load_tmx_map("assets/tiled/map_0.tmx").unwrap()
}

pub fn main() {
    let _map = init_map();
    App::new()
        // Base Plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "NFTactics".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_plugins(bevy_ecs_tilemap_helper_tiled::TiledMapPlugin)
        // Resources
        .insert_resource(resources::settings::Volume(50))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Splash)
                .load_collection::<NFTacticsAssets>(),
        )
        .add_systems(Startup, startup)
        .add_plugins(splash_plugin)
        .add_plugins(main_menu_plugin)
        .add_systems(OnEnter(GameState::MainMenu), start_background_audio)
        .add_systems(OnEnter(GameState::Game), game_startup)
        .add_systems(Update, || {})
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn start_background_audio(
    mut commands: Commands,
    assets: Res<NFTacticsAssets>,
    volume: Res<resources::settings::Volume>,
) {
    commands.spawn(AudioBundle {
        source: assets.audio_nftactics_track_0.clone(),
        settings: PlaybackSettings::LOOP.with_volume(Volume::new(volume.0 as f32 / 100.0)),
    });
}

fn game_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<bevy_ecs_tilemap_helper_tiled::TiledMap> =
        asset_server.load("tiled/map_0.tmx");

    commands.spawn(bevy_ecs_tilemap_helper_tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
