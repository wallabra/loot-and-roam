// --------
// Part slots and slot matching.
// --------

use bevy::ecs::component::Component;

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
