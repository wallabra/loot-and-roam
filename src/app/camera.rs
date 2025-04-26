//! # General camera code
//!
//! Different camera types, such as the [PlayerCamera] and the
//! [DevCamera].

// Written by:
// * perospirone (https://codeberg.org/perospirone)
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

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

/// The player camera.
///
/// Cameras with this component will be instructed to follow the local instance
/// player ship, if one is available and queriable.
#[derive(Component)]
pub struct PlayerCamera;

/// Setups the player camera on the world.
///
/// Run whenever an island state is entered.
pub fn setup_camera(mut commands: Commands) {
    // [TODO] setup on entering game state
    // PREREQ: superstates (use bevy states)
    commands.spawn((Camera3d::default(), PlayerCamera));
}

fn player_camera_controller(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // [TODO] Add ship follow functionality
    // (PREREQ: ships, player data)

    for mut transform in query.iter_mut() {
        let mut move_direction = Vec3::ZERO;
        let speed = 5.0;

        // Basic WASD movement and space/shift for vertical movement
        if keyboard_input.pressed(KeyCode::KeyW) {
            move_direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            move_direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            move_direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            move_direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            move_direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            move_direction.y -= 1.0;
        }

        transform.translation += move_direction * speed * time.delta_secs();
    }
}

/// A debug camera, used in some examples to help navigate them.
#[derive(Component)]
pub struct DevCamera {
    /// Movement speed. Doubled when Ctrl is held.
    pub move_speed: f32,

    /// Mouse sensitivity.
    pub rotate_sensitivity: f32,

    /// Current rotation's pitch component.
    pub pitch: f32,

    /// Current rotation's yaw component.
    pub yaw: f32,

    /// Whether the dev camera is enabled.
    ///
    /// Inputs are only processed if this is true.
    pub enabled: bool,
}

impl Default for DevCamera {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            rotate_sensitivity: 0.1,
            pitch: 0.0,
            yaw: 0.0,
            enabled: true,
        }
    }
}

fn dev_camera_controller(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    _mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut DevCamera)>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut().unwrap();

    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;

    if let Ok((mut transform, mut controller)) = query.single_mut() {
        if !controller.enabled {
            return;
        }

        // Rotation via mouse
        let mut delta = Vec2::ZERO;
        for ev in mouse_motion_events.read() {
            delta += ev.delta;
        }

        controller.yaw -= delta.x * controller.rotate_sensitivity * time.delta_secs();
        controller.pitch -= delta.y * controller.rotate_sensitivity * time.delta_secs();
        controller.pitch = controller
            .pitch
            .clamp(-89.9f32.to_radians(), 89.9f32.to_radians());

        let yaw = Quat::from_rotation_y(controller.yaw);
        let pitch = Quat::from_rotation_x(controller.pitch);
        transform.rotation = yaw * pitch;

        // Movement via WASD
        let mut direction = Vec3::ZERO;
        let forward = transform.forward();
        let right = transform.right();

        if keys.pressed(KeyCode::KeyW) {
            direction += *forward;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction -= *forward;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += *right;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction -= *right;
        }
        if keys.pressed(KeyCode::KeyE) || keys.pressed(KeyCode::Space) {
            direction += Vec3::Y;
        }
        if keys.pressed(KeyCode::KeyQ) || keys.pressed(KeyCode::ShiftLeft) {
            direction -= Vec3::Y;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        transform.translation += direction * controller.move_speed * time.delta_secs();
    }
}

/// Camera control plugin.
///
/// Necessary in order to properly use [PlayerCamera] amd [DevCamera].
///
/// Included in [crate::app::AppPlugin].
pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_camera_controller, dev_camera_controller));
    }
}

pub mod prelude {
    pub use super::CameraControlPlugin;
    pub use super::DevCamera;
    pub use super::PlayerCamera;
}
