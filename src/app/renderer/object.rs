//! # Object rendering code.
//!
//! Objects in Loot & Roam are physically a network of points, with volumes
//! attached to them.
//!
//! This module contains code common to the rendering of all objects.

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

use bevy::prelude::*;

use crate::common::physics::base::PointNetwork;

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

pub struct ObjectRendererPlugin;

impl Plugin for ObjectRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_focus_system,));
    }
}
