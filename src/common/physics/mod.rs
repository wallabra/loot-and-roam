//! # Object physics engine.
//!
//! Objects in Loot & Roam are physically comprised of a network of "points",
//! which are mobile points in 3D space, to which can be attached physical
//! volumes, visual point-attached renderers, and springs between points.

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
// pub mod volume;    // Volumes, their intersection, and volume/surface forces
pub mod base; // Basic point network definitions and systems.
pub mod collision; // Advanced collision handling for objects
