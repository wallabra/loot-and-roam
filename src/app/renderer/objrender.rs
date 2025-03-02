//! # Object rendering code.
//!
//! Objects in Loot & Roam are physically a network of points, with volumes
//! attached to them, and visually PointRender instances, each attached to
//! a point.
//!
//! A PointRender component refers to exactly one point, and will always
//! snap to it. If the point ceases to exist, the entity destroys itself.
//!
//! A PointRender statically enum-dispatches the production of rendering
//! commands to one of its implementations:
//!
//! * PointSprite - a billboarded sprite snapped to the location of the point
//!
//! * PointModel - a model snapped to the location of the point

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
