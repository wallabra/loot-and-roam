//! # Middle-level rendering commands.
//!
//! All visual objects in Loot & Roam are composed of rendering components,
//! such as [PointRender], and every frame, rendering components quickly
//! produce a list of "rendering commands". Those are then sent to the GPU to
//! be rendered.
//!
//! These are unlike UI rendering commands, which are handled entirely on the
//! CPU. The 'UI' layer is always drawn on top of the 'game' layer.

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
