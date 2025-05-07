//! # API entrpoint for Loot & Roam.

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

pub mod app;
pub mod common;
pub mod server;

/// The main Loot & Roam plugin.
///
/// Enables every mandatory engine plugin.
pub struct LootAndRoamEnginePlugin;

impl Plugin for LootAndRoamEnginePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((common::CommonPlugin, app::AppPlugin, server::ServerPlugin));
    }
}

pub mod prelude {
    pub use super::app::prelude::*;
    pub use super::common::prelude::*;
    pub use super::server::prelude::*;
    pub use super::LootAndRoamEnginePlugin;
}
