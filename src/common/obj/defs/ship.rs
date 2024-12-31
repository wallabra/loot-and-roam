//! Ship-specific object behaviour.

use crate::common::shipmakeup::ShipMakeup;

/// A Ship,
///
/// Can be controlled by either a player or an AI.
///
/// It can use its engines to move around, take damage from various sources,
/// and drop its loot when destroyed. It can also shoot its cannons and attract
/// nearby items using its vacuums.
///
/// This object only refers to a simulated instance of a ship. Its definitions
/// are stored in a ShipMakeup owned by the Ship, and its physical properties
/// are handled in a separate system, the physics system.
pub struct Ship {
    pub makeup: ShipMakeup,
}

impl Ship {}
