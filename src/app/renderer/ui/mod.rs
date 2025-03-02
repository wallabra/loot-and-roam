//! # UI rendering & engine code
//!
//! Atop the 'game' layer, the 'ui' layer is drawn.
//!
//! The UI engine is used to handle both the menus and the HUD.
//!
//! First, on every frame, UI elements are generated stackwise according to the
//! current state. UI elements must be produced within an "UI context", which
//! groups UI elements and lays them out relative to each other. Different
//! elements of the game state, such as the superstate (e.g. main menu UI,
//! game state HUD, etc) or objects (e.g. ships displaying their names), will
//! have their own independent UI contexts.
//!
//! The layout engine takes every UI element inside an UI context, and produces
//! the final positions and sizes for every element, seeking to respect
//! constraints as much as possible. With those exact coordinates, it produces
//! a list of "UI commands", which are low-level commands used to actually
//! render the UI elements, such as "rectangle", "image", "text", etc.
//!
//! ## Immediate mode
//!
//! UI elements are generated immediate-mode, which means entities and
//! resources which wish to display UI (such as the supertate or ships) must
//! do so **every frame**, independently, without UI state being kept between
//! frames. They must implement "display logic", which takes their current
//! state and produces UI elements according to a passed API, and "update
//! logic", which takes in UI events and may update the internal state.

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

// [WIP] Please uncomment *only* implemented modules.
// pub mod layouter;
// pub mod event;
// pub mod builder;
