//! Part slots and slot matching.

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

/// Refers to a construct entity, of which this one is a part slot.
///
/// This logical relationship is used as opposed to direct parenting, to
/// decouple the physics and logical hierarchies.
#[derive(Component)]
#[relationship(relationship_target = ConstructSlots)]
pub struct SlotOfConstruct(Entity);

impl SlotOfConstruct {
    pub fn get(&self) -> Entity {
        self.0
    }

    pub fn new(construct_id: Entity) -> Self {
        Self(construct_id)
    }
}

impl Deref for SlotOfConstruct {
    type Target = Entity;

    fn deref(&self) -> &Entity {
        &self.0
    }
}

/// Lists the part slots this construct entity has.
#[derive(Component)]
#[relationship_target(relationship = SlotOfConstruct)]
pub struct ConstructSlots(Vec<Entity>);

impl ConstructSlots {
    pub fn iter(&self) -> std::slice::Iter<'_, bevy::prelude::Entity> {
        self.0.iter()
    }

    pub fn new(slot_ids: &[Entity]) -> Self {
        Self(Vec::from(slot_ids))
    }
}

/// Entity which can serve as a part slot.
///
/// Its parent will necessarily be a construct.
#[derive(Component)]
pub struct PartSlotInfo {
    /// The type of parts compatible with this slot.
    ///
    /// Multiple compatibility types cannot be specified for a single slot.
    /// However, a part may specify multiple compatibility tags. Therefore,
    /// slots of different types can be compatible with the same tag.
    pub slot_type: String,
}

/// A part which can be installed on a construct via one of its [`PartSlot`]s.
#[derive(Component)]
pub struct PartInfo {
    /// WHich [`PartSlot.slot_type`]s are compatible with this part.
    pub tags: Vec<String>,
}

//--- Public Utility Functions
/// Make a part slot component.
pub fn part_slot(slot_type: String) -> PartSlotInfo {
    PartSlotInfo { slot_type }
}

/// Make a part info component.
pub fn part_tags(tags: Vec<String>) -> PartInfo {
    PartInfo { tags }
}

/// Make a part info component with a single tag.
pub fn part_tag(tag: String) -> PartInfo {
    PartInfo { tags: vec![tag] }
}
