//! # Ship state and components.

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

use bevy::prelude::*;
use slotmap::{DefaultKey, SlotMap};

use super::inventory::InventoryDef;

// [TODO] Please uncomment *only* implemented modules.
// pub mod parts; // Ship parts.

/// Marks an entity as a ship.
#[derive(Component)]
pub struct Ship {
    /// The state of this ship.
    pub makeup: ShipMakeup,
}

/// A part slot.
///
/// Each [ShipMake] has a list of slots to which parts can be installed by
/// type.
pub struct PartSlot {
    /// The type of part that can be instlaled here.
    ///
    /// Par types for installation are defiend as keywords, such as "engine"
    /// or "cannon".
    pub part_type: String,

    /// The offset of the part installed on this slot.
    ///
    /// Every part knows which slot is it installed to, and is therefore
    /// rendered accordingly. This also informs behavior such as cannons firing
    /// cannonballs from a starting position offset by this.
    ///
    /// The offset is relative to the point the part is attached to, since all
    /// parts must be attached to a point on the point network.
    pub offset: Vec3,

    /// Point network attachment.
    ///
    /// Every part must be attached to a point network.
    pub point_attachment: usize,
}

/// The make of the ship.
///
// This defines the ship's base hull, as well as part slot definitions.
pub struct ShipMake {
    /// The hull mass.
    pub hull_mass: f32,

    /// Part slots.
    pub slots: Vec<PartSlot>,
}

pub struct ShipMakeup {
    /// The make of this ship.
    make: ShipMake,

    /// A vector of part installation.
    ///
    /// Each index corresponds to a slot. Each item, when present, is an index
    /// into the inventory.
    parts: Vec<Option<DefaultKey>>,

    /// The inventory of this ship.
    ship_inventory: SlotMap<DefaultKey, InventoryDef>,
}

impl ShipMakeup {
    /// Sums up the total mass of the ship,
    pub fn get_total_mass(&self) -> f32 {
        self.make.hull_mass
            + self
                .ship_inventory
                .iter()
                .map(|(_, inv)| inv.mass * inv.amount)
                .sum::<f32>()
    }

    /// Iterate on all parts and their slots.
    pub fn part_iter(&self) -> impl Iterator<Item = (&InventoryDef, &PartSlot)> {
        self.parts
            .iter()
            .filter_map(|maybe_part| maybe_part.map(|part| self.ship_inventory.get(part).unwrap()))
            .zip(self.make.slots.iter())
    }
}
