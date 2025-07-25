//! # Constructs
//!
//! Composite objects which may have multi-functional parts, as well as slot
//! definitions for them.
//!
//! For more info, see: https://codeberg.org/GameCircular/loot-and-roam/issues/16

use bevy::prelude::*;

pub mod install;
pub mod part;
pub mod slot;

pub mod prelude {
    pub use super::install::{TryInstallPartOnConstruct, TryInstallPartOnSlot, TryUninstallPart};
    pub use super::part::{ConstructParts, PartInstalledOn};
    pub use super::slot::{PartInfo, PartSlotInfo, part_slot, part_tag, part_tags};
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
        app.add_observer(install::ev_try_install_part_on_slot);
        app.add_observer(install::ev_try_install_part_on_construct);
        app.add_observer(install::ev_try_uninstall_part);
    }
}
