//! # Loot & Roam common code.
//!
//! This contains primarily the simulation code, which is used primarily server-side,
//! but is also used by the client for client-side prediction (CSP).
//!
//! Bevy is the engine used to keep track of game state. However, some simulation
//! systems are their own self-contained Resources, rather than components or systems.
//! This is for performance reasons.
//!
//! A player can load a single island at a time. There are two types of islands:
//!
//! * Bases: those are the meat and potatoes of the game, where players spawn in to loot
//!   the props/buildings and the ships therein.
//!
//!   Here is where all the physics and combat happens.
//!
//! * Towns: presnted as an intermission screen (either a menu or a map with clickable
// !  buildings that enter menus, depending on client settings), where players can
// !  modify their ships and manage their inventory.
// !
// !  Being a sort of non-game state, towns are made interesting by means of an
// !  economic simulation.

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

use bevy::prelude::Plugin;

pub mod construct; // Constructs (genrealized part holders)
pub mod inventory; // Inventory items and related operations
pub mod makeup; // Ship makeup and parts
pub mod math; // Mathematical utility functions
pub mod physics; // Object physics and collision detection
pub mod scene; // Scene management and initializatoin
pub mod state; // Ingame state handling
pub mod terrain; // Terrain generation, caching, and lookup

// pub mod defs;      // Definitions for ship parts, makes, NPC templates, etc
// pub mod namegen;   // Localizable name generation for NPC ships
// pub mod ai;        // NPC ship controller
// pub mod player;    // Player state tracking
// pub mod spawner;   // NPC ship spawning
// pub mod props;     // Static props (decorative, buildings, etc) and their spawning
// pub mod town;      // Economic mechanisms, and town state tracking
// pub mod meta;      // Simulation meta-state, including game name, difficulty level, etc
// pub mod event;     // Top-level events (player creation, login, death, mooring, etc.)
// ṕub mod util;      // Miscellaneous utility functions

/// Main game plugin, groups all the important Loot & Roam systems together.
///
/// This is essential to be registered in any simulation instance, regardless
/// of it being headless or not.
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            physics::BasicPhysicsPlugin,
            terrain::collision::TerrainCollisionPlugin,
            state::BaseStatePlugin,
            scene::SceneManagementPlugin,
            physics::collision::CollisionPlugin,
            construct::ConstructPlugin,
        ));
    }
}

pub mod prelude {
    pub use super::CommonPlugin;
    pub use super::construct::prelude::*;
    pub use super::math::*;
    pub use super::physics::prelude::*;
    pub use super::terrain::prelude::*;
}
