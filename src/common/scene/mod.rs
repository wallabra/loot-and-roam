//! # Scene management
//!
//! The *scene* is the layout of an overworld game. Scene management
//! encompasses the initialization of an overworld scene: the players, the NPC
//! ships, and the island with its terrain and props, as well as the
//! maintenance of scene through visual effects and ambient noise.

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

pub mod init;

use bevy::prelude::Plugin;

/// Plugin that activates all scene management code.
pub struct SceneManagementPlugin;

impl Plugin for SceneManagementPlugin {
    fn build(&self, app: &mut bevy::app::App) {}
}

pub mod prelude {
    pub use super::SceneManagementPlugin;
}
