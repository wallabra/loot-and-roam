//! # Graphics & rendering code.
//!
//! While Bevy is the main game engine and handles the backend, more advanced
//! rendering functionality such as the lighting engine and terrain SDF3D
//! rendering can be found here.

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
// pub mod lighting;  // Scene lighting definitions
pub mod object; // Common object rendering code
pub mod sky; // Sky/background
pub mod terrain; // Terrain renderer
pub mod ui; // UI renderer

pub mod prelude {
    pub use super::sky::SkyRenderingPluginn;
}
