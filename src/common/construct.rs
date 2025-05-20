//! # Constructs
//!
//! Composite objects which may have multi-functional parts, as well as slot
//! definitions for them.
//!
//! For more info, see: https://codeberg.org/GameCircular/loot-and-roam/issues/16

use bevy::prelude::*;

/// Enables all generalized construct and construct part related behavior.
///
/// Already included in the [`CommonPlugin`].
pub struct ConstructPlugin;

impl Plugin for ConstructPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TryInstallPartOnSlot>();
        app.add_event::<TryInstallPartOnConstruct>();
        app.add_event::<TryUninstallPart>();
        app.add_observer(ev_try_install_part_on_slot);
        app.add_observer(ev_try_install_part_on_construct);
        app.add_observer(ev_try_uninstall_part);
    }
}

// --------
// Construct-part relationship.
// --------

/// Component present in all installed construct parts.
///
/// Wraps a reference to the construct to which this part is installed.
#[derive(Component)]
#[relationship(relationship_target = ConstructParts)]
pub struct PartInstalledOn(Entity);

/// Component present in all constructs.
///
/// Lists the parts that are currently installed on this construct.
#[derive(Component)]
#[relationship_target(relationship = PartInstalledOn)]
pub struct ConstructParts(Vec<Entity>);

// --------
// Construct-part change entrypoints.
// --------

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

fn ev_try_install_part_on_slot(
    trigger: Trigger<TryInstallPartOnSlot>,
    mut commands: Commands,
    parent_query: Query<&ChildOf>,
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
                "TryInstallPart triggered for a part slot with no or corrupted parent: {}",
                slot_query_err
            );
        }
        Ok(child_of) => child_of.0,
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
        let mut part = commands.entity(part_id);
        part.insert(PartInstalledOn(construct_id));
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

fn ev_try_install_part_on_construct(
    trigger: Trigger<TryInstallPartOnConstruct>,
    mut commands: Commands,
    installation_query: Query<&PartInstalledOn>,
    part_query: Query<&PartInfo>,
    slot_query: Query<&PartSlotInfo>,
    children_query: Query<&Children>,
) {
    let part_id = trigger.target();
    let mut part = commands.entity(part_id);
    assert!(!installation_query.contains(part_id));
    let part_info = part_query.get(part_id).unwrap();

    let event = trigger.event();
    let construct_id = event.which_construct;

    let available_slot: Option<Entity> = match children_query.get(construct_id) {
        Ok(children) => children.iter().find(|construct_child| {
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
                            .any(|slot_child| part_query.contains(slot_child))
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

fn ev_try_uninstall_part(
    trigger: Trigger<TryUninstallPart>,
    mut commands: Commands,
    parent_query: Query<&ChildOf>,
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
        let slot_id = parent_query.get(part_id).unwrap().0;
        assert!(slot_query.contains(slot_id));
        let mut slot = commands.entity(slot_id);

        let construct_id = installation_query.get(part_id).unwrap().0;
        assert_eq!(parent_query.get(slot_id).unwrap().0, construct_id);

        slot.remove_children(&[part_id]);
    }
}

// --------
// Part slots and slot matching.
// --------

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
    slot_type: String,
}

/// A part which can be installed on a construct via one of its [`PartSlot`]s.
#[derive(Component)]
pub struct PartInfo {
    /// WHich [`PartSlot.slot_type`]s are compatible with this part.
    tags: Vec<String>,
}
