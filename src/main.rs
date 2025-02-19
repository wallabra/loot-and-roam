//! # Application entrypoint for Loot & Roam.
//!
//! Every running program has in it a server and a client.
//!
//! On non-authoritative instances, the server exists for network balancing.
//! The client can connect to another instance to interact with the
//! authoritative instance; in authoritative instances, the client is merely
//! for player interaction and server control.
//!
//! The program can also be run "headless", that is, without displaying the
//! game.

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

use bevy::{ecs::world::DeferredWorld, prelude::*};

/// A Tokio runtime wrapped in a Bevy resource.
#[derive(Resource)]
struct TokioRuntime(pub(crate) tokio::runtime::Runtime);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TokioRuntime(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        ))
        .run();
}
