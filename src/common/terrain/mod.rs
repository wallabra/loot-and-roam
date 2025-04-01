//! # Terrain definitions
//!
//! Broadly speaking, terrain is defined as a tree of 'terrain primitives',
//! which generate fields such as the heightmap and signed distance field,
//! which are used in both the physics and rendering stages.

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

pub mod noise;

pub mod prelude {
    pub use super::noise::NoiseLattice;
}
