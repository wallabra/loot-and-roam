//! # Water physics
//!
//! Buoyancy, drag, and .

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
    forces::Gravity,
    volume::{VolumeCollection, VolumeInfo},
};

/// This Bevy component applies water physics to a physics-enabled object.
///
/// This includes both drag and buoyancy.
///
/// Requires ]PointNetwork] and [VolumeCollection].
#[derive(Component, Clone)]
pub struct WaterPhysics {
    /// Drag force factor.
    pub drag_factor: f32,

    /// Buoyancy factor.
    pub buoyancy_factor: f32,

    /// Y intercept of water level.
    ///
    /// All geometry below this point is considered submerged.
    pub water_level: f32,
}

impl Default for WaterPhysics {
    fn default() -> Self {
        Self {
            drag_factor: 0.5,
            buoyancy_factor: 0.5,
            water_level: 0.0,
        }
    }
}

/// The system responsible for water drag in the physics system.
fn water_drag_system(
    time: Res<Time>,
    mut query: Query<(&mut PointNetwork, &VolumeCollection, &WaterPhysics)>,
) {
    for (mut points, volumes, water_physics) in query.iter_mut() {
        for volume in &volumes.volumes {
            let point = &mut points.points[volume.point_idx];

            // [NOTE] Water level is fixed to the Y axis because of the
            // geometry API only requiring volume_below and surface_below.
            let water_area = volume
                .volume_type
                .surface_area_below(water_physics.water_level - point.pos.y);

            if water_area <= 0.0 {
                continue;
            }

            let drag = -point.vel * water_area * water_physics.drag_factor;
            point.apply_force_over_time(drag, time.delta_secs());
        }
    }
}

/// The system responsible for buoyancy in the physics system.
fn water_buoyancy_system(
    time: Res<Time>,
    mut query: Query<(
        &mut PointNetwork,
        &VolumeCollection,
        &WaterPhysics,
        &Gravity,
    )>,
) {
    for (mut points, volumes, water_physics, gravity) in query.iter_mut() {
        for volume in &volumes.volumes {
            let point = &mut points.points[volume.point_idx];

            // [NOTE] Water level is fixed to the Y axis because of the
            // geometry API only requiring volume_below and surface_below.
            let water_vol = volume
                .volume_type
                .volume_below(water_physics.water_level - point.pos.y);

            if water_vol <= 0.0 {
                continue;
            }

            // 1 m³ of water = 0.997 kg, conveniently
            let water_displaced_kg = water_vol * 0.997;
            let buoyancy = -gravity.force * water_displaced_kg * water_physics.buoyancy_factor;

            point.apply_force_over_time(buoyancy, time.delta_secs());
        }
    }
}

pub struct WaterPhysicsPlugin;

impl Plugin for WaterPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (water_drag_system, water_buoyancy_system));
    }
}
