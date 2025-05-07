//! # On-game application state
//!
//! Sets up the internal game logic (pretty much everything in the `common`
//! tree).

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

use bevy::prelude::*;

use crate::common::state::GameState;

use super::AppState;

fn ingame_state_exit(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::None);
}

pub struct AppInGameStatePlugin;

impl Plugin for AppInGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::InGame), ingame_state_exit);
    }
}
