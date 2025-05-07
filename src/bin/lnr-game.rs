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

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use loot_and_roam::LootAndRoamEnginePlugin;

fn main() {
    let mut app = App::new();

    // default plugin & main properties
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Loot & Roam".into(),
            name: Some("bevy.loot-and-roam".into()),
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }));

    // framerate
    // app.add_plugins(FrameTimeDiagnosticsPlugin::default());

    // engine
    app.add_plugins(LootAndRoamEnginePlugin);

    // logger
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.run();
}
