use bevy::prelude::*;

static CONFIG: &str = r#"
    name = "NFTactics"
    version = "0.1.0"
    author = "Mike Hukiewitz, David Alexander Pfeiffer"
    description = "Next-gen web3 auto battler"
    [settings]
    resolution = [256, 144]
    [solana]
    http-rpc-url = "http://127.0.0.1:8899"
    ws-rpc-url = "ws://127.0.0.1:8900"
"#;

fn init_map() -> tiled::Map {
    let mut loader = tiled::Loader::new();

    loader.load_tmx_map("tiled/map_0.tmx").unwrap()
}

pub fn main() {
    let map = init_map();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, || {})
        .run();
}
