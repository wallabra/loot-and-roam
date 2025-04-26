//! # The official Loot & Roam client.
//!
//! Contains code for displaying the game, interacting with user input,
//! loading client-side assets, handling in-game audio, and so on.

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

// [TODO] Please uncomment *only* implemented modules.
// pub mod audio;
// pub mod resource;
// pub mod input;
pub mod camera; // Camera controls & updates
pub mod renderer; // Rendering code

/// Loot & Roam app plugin.
///
/// Applies every application system. Can be left out for 'headless'
/// configurations.
pub struct AppPlugin;

impl bevy::prelude::Plugin for AppPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((renderer::RendererPlugin, camera::CameraControlPlugin));
    }
}

pub mod prelude {
    pub use super::camera::prelude::*;
    pub use super::renderer::prelude::*;
    pub use super::AppPlugin;
}
