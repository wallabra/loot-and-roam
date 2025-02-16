//! # Server-side code.
//!
//! Loot & Roam uses a distributive-authoritative model, where a single instance is declared as "authoritative", and
//! other instances try to predict it.
//!
//! Every instance has their own internal server. Authoritative instances run the simulation in it. All internal
//! servers also distribute network events, on a shoot-first, ask-later basis (i.e. without keeping track of which
//! instances already received them, but ignoring already-received packets).

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
