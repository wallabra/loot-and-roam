//! # Object physics engine
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

use base::{point_attach_snap, point_base_physics};
use bevy::prelude::*;
use forces::BasicForcesPlugin;
use spring::SpringForcesPlugin;
use water::WaterPhysicsPlugin;

pub mod base; // Basic point network definitions and systems
pub mod collision; // Advanced collision handling for objects
pub mod forces; // Basic forces
pub mod spring; // Spring based soft body implementation
pub mod torque; // User rotational forces
pub mod volume; // Volumes, their intersection, and volume/surface forces
pub mod water; // Water physics

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
        app.add_systems(
            FixedUpdate,
            (
                point_base_physics,
                point_attach_snap.after(point_base_physics),
            ),
        );
        app.add_plugins((SpringForcesPlugin, BasicForcesPlugin, WaterPhysicsPlugin));
    }
}

pub mod prelude {
    pub use super::BasicPhysicsPlugin;
    pub use super::base::{PhysPoint, PointAttach, PointNetwork};
    pub use super::collision::{
        CollisionPlugin, FloorPlaneCollision, VolumeVolumeCollisionDetectionEvent,
    };
    pub use super::forces::{AirDrag, Gravity};
    pub use super::spring::{NormalSpring, Spring, SpringMode, SpringNetwork};
    pub use super::volume::{
        AABB, CollisionInfo, PhysicsVolume, SphereDef, VolumeCollection, VolumeCollision,
        VolumeInfo, VolumeType,
    };
    pub use super::water::WaterPhysics;
}
