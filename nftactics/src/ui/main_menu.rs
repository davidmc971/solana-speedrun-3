use bevy::prelude::*;

use crate::GameState;

pub fn main_menu_plugin(app: &mut App) {
    app.add_systems(Update, placeholder.run_if(in_state(GameState::MainMenu)));
}

fn placeholder(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Game);
}
