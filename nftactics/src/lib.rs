turbo::cfg! {r#"
    name = "NFTactics"
    version = "0.1.0"
    author = "Mike Hukiewitz, David Alexander Pfeiffer"
    description = "Next-gen web3 auto battler"
    [settings]
    resolution = [256, 144]
    [solana]
    http-rpc-url = "http://127.0.0.1:8899"
    ws-rpc-url = "ws://127.0.0.1:8900"
"#}

turbo::init! {
    // Define the GameState struct.
    struct GameState {
        screen: enum Screen {
            Title,
            Level,
        },
        x_position: i32,
        y_position: i32,
    } = {
        // Set the struct's initial value.
        Self {
            screen: Screen::Title,
            x_position: 30,
            y_position: 40,
        }
    }
}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! {
    // 1. Load State
    // This hydrates state from the previous loop.
    // The initial state's initial value will load on the first loop.
    let mut state = GameState::load();

    clear(0x121314ff);

    // 2. Update State
    // Your game's logic goes here. Mutate state as-needed.
    text!("Hello, world!!!");

    // 3. Save State
    // The final thing you should do in your game loop is save your game state.
    // This serializes state and persists it in memory so it doesn't get lost while hot-reloading
    state.save();
}
