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

use base::{gravity, point_base_physics};
use bevy::prelude::*;
use spring::point_spring_forces;

pub mod base; // Basic point network definitions and systems
pub mod collision; // Advanced collision handling for objects
pub mod spring; // Spring based soft body implementation
pub mod volume; // Volumes, their intersection, and volume/surface forces

/// # Basic physics
///
/// Adds systems for basic physics:
///
/// * Point inertia (applying velocity to position) - see [PointNetwork].
/// * [SpringNetwork]s.
/// * [Gravity].
pub struct BasicPhysicsPlugin;

impl Plugin for BasicPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (point_base_physics, point_spring_forces, gravity));
    }
}

pub mod prelude {
    pub use super::base::{Gravity, PhysPoint, PointNetwork};
    pub use super::collision::{CollisionPlugin, FloorPlaneCollision};
    pub use super::spring::{NormalSpring, Spring, SpringMode, SpringNetwork};
    pub use super::volume::{
        CollisionInfo, PhysicsVolume, SphereDef, VolumeCollection, VolumeCollision, VolumeInfo,
        VolumeType, AABB,
    };
    pub use super::BasicPhysicsPlugin;
}
