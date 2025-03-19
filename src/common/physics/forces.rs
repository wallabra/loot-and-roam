//! # Basic physics forces
//!
//! Gravity and air drag.

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

use super::{
    base::PointNetwork,
    volume::{VolumeCollection, VolumeInfo},
};

/// This Bevy component applies gravity to a physics-enabled object.
///
/// Requires ]PointNetwork].
#[derive(Component)]
pub struct Gravity {
    /// The force of gravity, with direction and magnitude.
    ///
    /// Defaults to -10 in the Y axis.
    pub force: Vec3,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            force: Vec3::Y * -10.0,
        }
    }
}

impl Gravity {
    pub fn new(gravity_force: Vec3) -> Self {
        Self {
            force: gravity_force,
        }
    }
}

/// The system responsible for gravity in the physics system.
fn gravity(time: Res<Time>, mut query: Query<(&mut PointNetwork, &Gravity)>) {
    for (mut points, gravity) in query.iter_mut() {
        for point in points.points.iter_mut() {
            point.vel += gravity.force * time.delta_secs();

            // Position change integration (approximating as that of a continuous linear acceleration)
            point.pos += 0.5 * gravity.force * time.delta_secs().powi(2);
        }
    }
}

/// This Bevy component applies air drag to a physics-enabled object.
///
/// Requires [PointNetwork] and [VolumeCollection].
#[derive(Component)]
pub struct AirDrag {
    pub drag_factor: f32,
}

impl AirDrag {
    pub fn new(drag_factor: f32) -> Self {
        Self { drag_factor }
    }
}

impl Default for AirDrag {
    fn default() -> Self {
        Self { drag_factor: 0.1 }
    }
}

/// The system responsible for air drag in the physics system.
fn air_drag(time: Res<Time>, mut query: Query<(&mut PointNetwork, &VolumeCollection, &AirDrag)>) {
    for (mut points, volumes, drag) in query.iter_mut() {
        for volume in &volumes.volumes {
            let point = &mut points.points[volume.point_idx];

            let drag = -point.vel * volume.volume_type.surface_area() * drag.drag_factor;
            point.apply_force_over_time(drag, time.delta_secs());
        }
    }
}

pub struct BasicForcesPlugin;

impl Plugin for BasicForcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (gravity, air_drag));
    }
}
