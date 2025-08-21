//! # Main menu state.
//!
//! Entering this state creates and displays a main menu to the screen.

// Written by:
// * Gustavo Ramos Rehermann <rehermann6046@gmail.com>
//
// (c)2025 GameCircular. Under the Cooperative Non-Violent Public License.
//
// Loot & Roam is non-violent software: you can use, redistribute,
// and/or modify it under the terms of the CNPLv6+ as found
// in the LICENSE file in the source code root directory or
// at <https://git.pixie.town/thufie/CNPL>.
//
// Loot & Roam comes with ABSOLUTELY NO WARRANTY, to the extent
// permitted by applicable law.  See the CNPL for details.

use bevy::{prelude::*, window::PrimaryWindow};

use crate::common::state::GameState;

use super::AppState;

#[derive(Component)]
struct MainMenuMarker;

fn main_menu_setup(mut commands: Commands, mut next_game_state: ResMut<NextState<GameState>>) {
    info!("Setting up main menu");
    next_game_state.set(GameState::None);
    commands.spawn((
        MainMenuMarker,
        Text2d("Loot & Roam".to_owned()),
        TextFont {
            font_size: 20.0,
            ..Default::default()
        },
        Transform::default(),
    ));
}

fn main_menu_cleanup(
    mut commands: Commands,
    q_mainmenu: Query<Entity, self::With<MainMenuMarker>>,
) {
    for e_mainmenu in q_mainmenu {
        commands.entity(e_mainmenu).despawn();
    }
}

fn input_handler_main_menu(
    keys: Res<ButtonInput<KeyCode>>,
    // TODO: use when implementing main menu
    _mouse_buttons: Res<ButtonInput<MouseButton>>,
    // TODO: use when implementing main menu
    _q_windows: Query<&Window, With<PrimaryWindow>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        info!("Leaving main menu for GameState::Start");
        next_game_state.set(GameState::Start);
        next_app_state.set(AppState::InGame);
    }
}

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), main_menu_setup);
        app.add_systems(OnExit(AppState::MainMenu), main_menu_cleanup);

        app.add_systems(
            Update,
            input_handler_main_menu.run_if(in_state(AppState::MainMenu)),
        );
    }
}
