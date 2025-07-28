//! # Constructs
//!
//! Composite objects which may have multi-functional parts, as well as slot
//! definitions for them.
//!
//! For more info, see: https://codeberg.org/GameCircular/loot-and-roam/issues/16

use bevy::prelude::*;

pub mod action;
pub mod install;
pub mod part;
pub mod slot;

pub mod prelude {
    pub use super::action::{
        DebugPrintPart, PartAction, PartActionDispatchRequest, dispatch_action,
    };
    pub use super::install::{TryInstallPartOnConstruct, TryInstallPartOnSlot, TryUninstallPart};
    pub use super::part::{ConstructParts, PartInstalledOn};
    pub use super::slot::{
        ConstructSlots, PartInfo, PartSlotInfo, SlotOfConstruct, part_slot, part_tag, part_tags,
    };
}

/// Enables all generalized construct and construct part related behavior.
///
/// Already included in the [`CommonPlugin`].
pub struct ConstructPlugin;

impl Plugin for ConstructPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<install::TryInstallPartOnSlot>();
        app.add_event::<install::TryInstallPartOnConstruct>();
        app.add_event::<install::TryUninstallPart>();
        app.add_event::<action::PartAction>();
        app.add_event::<action::PartActionDispatchRequest>();
        app.add_systems(Update, action::ev_dispatch_part_actions);
        app.add_observer(install::ev_try_install_part_on_slot);
        app.add_observer(install::ev_try_install_part_on_construct);
        app.add_observer(install::ev_try_uninstall_part);
        app.add_observer(action::obs_debug_part_action);
    }
}
