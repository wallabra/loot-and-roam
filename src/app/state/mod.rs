//! # App states.
//!
//! The main game application can be in the menu, or in the game.

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

pub mod ingame;
pub mod mainmenu;

/// The applicaiton state of the game.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    /// The application is in the main menu.
    ///
    /// Submenu states are handled by the menu UI stack resource.
    #[default]
    MainMenu,

    /// The application is currently in the game.
    ///
    /// Only in this state are game systems active, because GameState is always
    /// None outside of AppState's InGame, at least in the Loot & Roam
    /// application.
    InGame,
}

/// Plugim that enables all application state behavior.
pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();

        app.add_plugins((mainmenu::MainMenuStatePlugin, ingame::AppInGameStatePlugin));
    }
}

pub mod prelude {
    pub use super::AppState;
    pub use super::AppStatePlugin;
}
