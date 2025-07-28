//! A minimal construct action dispatch test, for debugging.

use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use loot_and_roam::prelude::*;

// Spawn a construct with 3 debug parts
fn setup(mut commands: Commands) {
    let construct = commands.spawn(Name::new("TestConstruct")).id();

    // Add slots
    let slot1 = commands.spawn(part_slot("debug".into())).id();
    let slot2 = commands.spawn(part_slot("debug".into())).id();
    let slot3 = commands.spawn(part_slot("noisy".into())).id();
    commands
        .entity(construct)
        .add_related::<SlotOfConstruct>(&[slot1, slot2, slot3]);
    // .insert(ConstructSlots::new(&[slot1, slot2, slot3]));

    // Install parts
    let part1 = commands
        .spawn(part_tags(["debug".into(), "extra".into()].into()))
        .id();
    let part2 = commands.spawn(part_tag("debug".into())).id();
    let part3 = commands.spawn(part_tag("noisy".into())).id();

    {
        commands
            .entity(part1)
            .trigger(TryInstallPartOnSlot::on(slot1));
        commands
            .entity(part2)
            .trigger(TryInstallPartOnSlot::on(slot2));
        commands
            .entity(part3)
            .trigger(TryInstallPartOnSlot::on(slot3));
    }

    send_debug_action(construct, &mut commands);
}

// Trigger action - will only b sent to parts with "debug" tag
fn send_debug_action(construct: Entity, commands: &mut Commands) {
    dispatch_action(
        commands,
        construct,
        "DEBUG".into(),
        vec!["debug".into()],
        Box::new(DebugPrintPart::with_message("Hello parts!")),
    );
}

fn apply_example_systems(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn main() {
    let mut app = App::new();

    // default plugin & main properties
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Loot & Roam Tech Demo - Construct Action Dispatch Test".into(),
            name: Some("bevy.loot-and-roam.test.construct-action-dispatch".into()),
            ..default()
        }),
        ..default()
    }));

    // engine systems
    app.add_plugins((CommonPlugin, AppPlugin));

    // system registration
    apply_example_systems(&mut app);

    // logger
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.run();
}
