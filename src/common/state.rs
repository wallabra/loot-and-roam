//! # State handling
//!
//! A game can be on the 'overworld' (an island raid), or on the 'intermission'
//! (shopping or managing the fleet). Bevy states track the different states.
//!
//! We also use Bevy's OnEnter events to perform initialization specific to
//! these states.

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

use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

/// The current superstate of the game.
///
/// A game typically cycles between:
///
/// * **Island raids**. These are internally known as the 'overworld'.
///
/// * The **intermission**, to manage the fleet and access external interfaces
///   like the Shop.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// Not in-game.
    ///
    /// This is the GameState for Bevy Apps that are not currently running a
    /// game, such as clients in the main menu.
    #[default]
    None,

    /// The very beginning of the game, before any island raid or intermission.
    ///
    /// The player can setup their character here, along with other optional
    /// roleplaying setup. They can also adjust their starting ship slightly,
    /// before setting sail for the very first time.
    ///
    // [NOTE] We should consider adding an intro cutscene here :D
    Start,

    /// The overworld state. THe meat and potatoes of the game, all the
    /// interesitng simulation happens in it.
    Overworld,

    /// The intemrission state. Lets players manage any aspects of the fleet
    /// that can't be managed on high water (such as replacing parts), and
    /// access the broader economy (such as through the Shop screen).
    Intermission,
}

#[derive(Component, Clone, Debug, Copy, Default)]
pub struct SceneTree;

#[derive(Clone, Debug, Event, Copy)]
pub struct SceneSetupEvent {
    pub scene_tree: Entity,
}

impl SceneSetupEvent {
    pub fn new(scene_tree: Entity) -> Self {
        Self { scene_tree }
    }
}

#[derive(Clone, Debug, Event, Default, Copy)]
pub struct SceneCleanup;

fn make_scene_tree(commands: &mut Commands) -> Entity {
    commands
        .spawn((SceneTree, Visibility::Visible, Transform::default()))
        .id()
}

fn setup_start(mut commands: Commands, mut ev_scene_setup: EventWriter<SceneSetupEvent>) {
    let tree = make_scene_tree(&mut commands);
    info!("Sending SceneSetup event for the Start state");
    ev_scene_setup.write(SceneSetupEvent::new(tree));
}

fn setup_overworld(mut commands: Commands, mut ev_scene_setup: EventWriter<SceneSetupEvent>) {
    let tree = make_scene_tree(&mut commands);
    info!("Sending SceneSetup event for the Overworld state");
    ev_scene_setup.write(SceneSetupEvent::new(tree));
}

fn setup_intermission(mut commands: Commands, mut ev_scene_setup: EventWriter<SceneSetupEvent>) {
    let tree = make_scene_tree(&mut commands);
    info!("Sending SceneSetup event for the Intermission state");
    ev_scene_setup.write(SceneSetupEvent::new(tree));
}

fn cleanup_start(mut commands: Commands, q_tree: Query<(Entity, &SceneTree)>) {
    commands.entity(q_tree.single().unwrap().0).despawn();
}

fn cleanup_overworld(mut commands: Commands, q_tree: Query<(Entity, &SceneTree)>) {
    commands.entity(q_tree.single().unwrap().0).despawn();
}

fn cleanup_intermission(mut commands: Commands, q_tree: Query<(Entity, &SceneTree)>) {
    commands.entity(q_tree.single().unwrap().0).despawn();
}

fn input_handler_start(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        info!("Start state received request to transition to Overworld");
        next_state.set(GameState::Overworld);
    }
}

fn input_handler_overworld(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mouse_motion_events: EventReader<MouseMotion>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyL) {
        info!("Overworld state received request to transition to Intermission");
        next_state.set(GameState::Intermission);
    }
}

fn input_handler_intermission(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyL) {
        info!("Intermission state received request to transition to Overworld");
        next_state.set(GameState::Overworld);
    }
}

/// Activates the main superstate systems.
///
/// This component is essential in Loot & Roam game execution.
pub struct BaseStatePlugin;

impl Plugin for BaseStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), setup_start);
        app.add_systems(OnEnter(GameState::Overworld), setup_overworld);
        app.add_systems(OnEnter(GameState::Intermission), setup_intermission);

        app.add_systems(OnExit(GameState::Start), cleanup_start);
        app.add_systems(OnExit(GameState::Overworld), cleanup_overworld);
        app.add_systems(OnExit(GameState::Intermission), cleanup_intermission);

        app.add_systems(
            Update,
            (
                input_handler_start.run_if(in_state(GameState::Start)),
                input_handler_overworld.run_if(in_state(GameState::Overworld)),
                input_handler_intermission.run_if(in_state(GameState::Intermission)),
            ),
        );

        app.init_state::<GameState>();

        app.add_event::<SceneSetupEvent>();
    }
}
