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

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::common::physics::base::PointNetwork;

/// Use this component on a child entity to attach it to a physics point of its parent.
///
/// The parent must have a [PointNetwork] component.
#[derive(Component)]
pub struct PointAttach {
    /// The index of the physics point on the parent's [PointNetwork].
    pub point_idx: usize,
}

fn point_attach_snap(
    mut query_child: Query<(&Parent, &mut Transform, &PointAttach)>,
    query_parent: Query<(&PointNetwork, &GlobalTransform, &Transform), Without<PointAttach>>,
) {
    for (parent, mut transform, attachment) in query_child.iter_mut() {
        let (parent_points, parent_global_transform, parent_transform) =
            query_parent.get(parent.get()).unwrap();

        assert!(attachment.point_idx < parent_points.points.len());

        transform.translation =
            parent_points.points[attachment.point_idx].pos - parent_global_transform.translation();
        transform.rotate_around(Vec3::ZERO, parent_transform.rotation.inverse());
    }
}

/// Camera target component.
#[derive(Component, Default)]
pub struct CameraFocus {
    /// Focus priority, highest value is used to point camera at.
    pub prio: f32,
}

fn camera_focus_system(
    mut cam_query: Query<&mut Transform, With<Camera3d>>,
    focus_query: Query<(&CameraFocus, &Transform), Without<Camera3d>>,
) {
    let mut focus = focus_query.iter().collect::<Vec<_>>();

    if focus.is_empty() {
        return;
    }

    focus.sort_by(|a, b| b.0.prio.partial_cmp(&a.0.prio).unwrap());
    let focus = focus[0].1;

    for mut cam_transform in cam_query.iter_mut() {
        cam_transform.look_at(focus.translation, Vec3::Y);
    }
}

#[derive(Component)]
pub struct DevCamera {
    pub move_speed: f32,
    pub rotate_sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub enabled: bool,
}

pub fn camera_controller_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut DevCamera)>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;

    if let Ok((mut transform, mut controller)) = query.get_single_mut() {
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

pub struct ObjectRendererPlugin;

impl Plugin for ObjectRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (point_attach_snap, camera_focus_system));
    }
}
