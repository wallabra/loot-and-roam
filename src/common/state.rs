//! # State handling
//!
//! A game can be on the 'overworld' (an island raid), or on the 'intermission'
//! (shopping or managing the fleet). Bevy states track the different states.
//!
//! We also use Bevy's OnEnter events to perform initialization specific to
//! these states.

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

/// The current superstate of the game.
///
/// A game is either:
///
/// * On an **island raid**. This is internally known as the 'overworld'.
///
/// * In an **intermission** - managing the fleet and accessing external
/// interfaces like the Shop.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// The very beginning of the game, before any island raid or intermission.
    ///
    /// The player can setup their character here, along with other optional
    /// roleplaying setup. They can also adjust their starting ship slightly,
    /// before setting sail for the very first time.
    ///
    // [NOTE] We should consider adding an intro cutscene here :D
    #[default]
    Start,

    /// The overworld state. THe meat and potatoes of the game, all the
    /// interesitng simulation happens in it.
    Overworld,

    /// The intemrission state. Lets players manage any aspects of the fleet
    /// that can't be managed on high water (such as replacing parts), and
    /// access the broader economy (such as through the Shop screen).
    Intermission,
}
