use bevy::prelude::*;

use crate::GameState;

use super::utils::despawn_screen;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct UIELement;

#[derive(Component)]
pub struct UIElementPlacableUnits;

pub fn interface_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), build_game_interface)
        .add_systems(
            Update,
            update_game_interface.run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnGameScreen>);
}

pub fn update_game_interface() {}

pub fn build_game_interface(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            (OnGameScreen, UIELement),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "NFTactics",
                TextStyle {
                    font_size: 32.0,
                    ..Default::default()
                },
            ));
        });
}
