//! Construct-part change entrypoints.

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

use bevy::ecs::{
    entity::Entity,
    event::Event,
    observer::Trigger,
    system::{Commands, Query},
};

use crate::common::construct::{
    part::PartInstalledOn,
    slot::{ConstructSlots, PartInfo, PartSlotInfo, SlotOfConstruct},
};

/// Event request to install a part onto a Construct on a givne slot.
///
/// This event must be targeted on the part.
///
/// May panic if the part is already installed to a construct or the slot
/// does not match the part.
#[derive(Event)]
pub struct TryInstallPartOnSlot {
    /// Which slot to install this part onto.
    ///
    /// The referred to entity must have a [`PartSlotInfo`], and must have a
    /// [`Parent`] - the construct onto which the part should be installed.
    which_slot: Entity,
}

impl TryInstallPartOnSlot {
    pub fn on(which_slot: Entity) -> Self {
        Self { which_slot }
    }
}

pub fn ev_try_install_part_on_slot(
    trigger: Trigger<TryInstallPartOnSlot>,
    mut commands: Commands,
    parent_query: Query<&SlotOfConstruct>,
    installation_query: Query<&PartInstalledOn>,
    part_query: Query<&PartInfo>,
    slot_query: Query<&PartSlotInfo>,
) {
    let part_id = trigger.target();
    assert!(!installation_query.contains(part_id));

    let event = trigger.event();
    let slot_id = event.which_slot;

    let construct_id = match parent_query.get(slot_id) {
        Err(slot_query_err) => {
            panic!(
                "TryInstallPart triggered for a part slot with no or corrupted construct reference: {}",
                slot_query_err
            );
        }
        Ok(child_of) => child_of.get(),
    };
    let part_info = part_query.get(part_id).unwrap();
    let slot_info = slot_query.get(slot_id).unwrap();

    if !part_info.tags.contains(&slot_info.slot_type) {
        panic!(
            "Tried to install part {:?} (with tags [{}]) onto slot {:?} (of type {})",
            part_id,
            part_info.tags.join(", "),
            slot_id,
            slot_info.slot_type
        );
    }

    {
        commands.entity(construct_id).add_one_related::<PartInstalledOn>(part_id);
    }

    {
        let mut slot = commands.entity(slot_id);
        slot.add_child(part_id);
    }
}

/// Event request to install a part onto a Construct on any vacant and matching
/// slot. The first slot found will be installed on, instead of any specific
/// slot.
///
/// This event must be targeted on the part.
///
/// May panic if the part is already installed to a construct or there are no
/// vacant matching slots on the referred to construct.
#[derive(Event)]
pub struct TryInstallPartOnConstruct {
    /// Which construct to install this part onto.
    ///
    /// The referred to entity must have at least one child entity with
    /// [`PartSlotInfo`] which is compatible with the targeted part and vacant
    /// (lacks children with are parts, i.e. bear [`PartInfo`]).
    which_construct: Entity,
}

impl TryInstallPartOnConstruct {
    pub fn on(which_construct: Entity) -> Self {
        Self { which_construct }
    }
}

pub fn ev_try_install_part_on_construct(
    trigger: Trigger<TryInstallPartOnConstruct>,
    mut commands: Commands,
    installation_query: Query<&PartInstalledOn>,
    part_query: Query<&PartInfo>,
    slot_query: Query<&PartSlotInfo>,
    children_query: Query<&ConstructSlots>,
) {
    let part_id = trigger.target();
    let mut part = commands.entity(part_id);
    assert!(!installation_query.contains(part_id));
    let part_info = part_query.get(part_id).unwrap();

    let event = trigger.event();
    let construct_id = event.which_construct;

    let available_slot: Option<Entity> = match children_query.get(construct_id) {
        Ok(children) => children.iter().copied().find(|construct_child| {
            if let Ok(slot_info) = slot_query.get(*construct_child) {
                // this is a part slot

                // skip if incompatible
                if !part_info.tags.contains(&slot_info.slot_type) {
                    return false;
                }

                // skip if not vacant
                children_query
                    .get(*construct_child)
                    .map(|slot_children| {
                        !slot_children
                            .iter()
                            .any(|slot_child| part_query.contains(*slot_child))
                    })
                    .unwrap_or(true)
            } else {
                false
            }
        }),
        _ => None,
    };

    match available_slot {
        Some(slot_id) => {
            part.trigger(TryInstallPartOnSlot {
                which_slot: slot_id,
            });
        }
        None => {
            panic!(
                "No available slot found on construct {:?} for part {:?}",
                construct_id, part_id
            );
        }
    }
}

/// Event request to uninstall a Part from its Construct.
///
/// Must be targeted on a part which contains [`PartInstalledOn`].
#[derive(Event)]
pub struct TryUninstallPart;

pub fn ev_try_uninstall_part(
    trigger: Trigger<TryUninstallPart>,
    mut commands: Commands,
    parent_query: Query<&SlotOfConstruct>,
    part_query: Query<&PartInfo>,
    slot_query: Query<&PartSlotInfo>,
    installation_query: Query<&PartInstalledOn>,
) {
    let part_id = trigger.target();
    assert!(part_query.contains(part_id));

    {
        let mut part = commands.entity(part_id);
        part.remove::<PartInstalledOn>();
    }

    {
        let slot_id = parent_query.get(part_id).unwrap().get();
        assert!(slot_query.contains(slot_id));
        let mut slot = commands.entity(slot_id);

        let construct_id = installation_query.get(part_id).unwrap().get();
        assert_eq!(parent_query.get(slot_id).unwrap().get(), construct_id);

        slot.remove_children(&[part_id]);
    }
}

/// Request the installation of a part on a slot.
///
/// Wraps around [TryInstallPartOnSlot].
pub fn install_part_on_slot(commands: &mut Commands, part: Entity, slot: Entity) {
    commands
        .entity(part)
        .trigger(TryInstallPartOnSlot::on(slot));
}

/// Request the installation of a part on a construct.
///
/// Wraps around [TryInstallPartOnConstruct].
pub fn install_part_on_construct(commands: &mut Commands, part: Entity, construct: Entity) {
    commands
        .entity(part)
        .trigger(TryInstallPartOnConstruct::on(construct));
}

/// Request the uninstallation of a part.
///
/// Wraps around [TryUninstallPart].
pub fn uninstall_part(commands: &mut Commands, part: Entity) {
    commands.entity(part).trigger(TryUninstallPart);
}
