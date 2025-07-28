//! Construct-part relationship.

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

use std::ops::Deref;

use bevy::ecs::{component::Component, entity::Entity};

/// Component present in all installed construct parts.
///
/// Wraps a reference to the construct to which this part is installed.
#[derive(Component)]
#[relationship(relationship_target = ConstructParts)]
pub struct PartInstalledOn(Entity);

impl PartInstalledOn {
    pub fn get(&self) -> Entity {
        self.0
    }

    pub fn new(construct_id: Entity) -> Self {
        Self(construct_id)
    }
}

impl Deref for PartInstalledOn {
    type Target = Entity;

    fn deref(&self) -> &Entity {
        &self.0
    }
}

/// Component present in all constructs.
///
/// Lists the parts that are currently installed on this construct.
#[derive(Component)]
#[relationship_target(relationship = PartInstalledOn)]
pub struct ConstructParts(Vec<Entity>);

impl ConstructParts {
    pub fn iter(&self) -> std::slice::Iter<'_, bevy::prelude::Entity> {
        self.0.iter()
    }

    pub fn new(slot_ids: &[Entity]) -> Self {
        Self(Vec::from(slot_ids))
    }
}
