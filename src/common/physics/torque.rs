//! # Torque application methods
//!
//! Extends PointNetwork with the ability to apply rotational force, aka
//! torque, to its points.

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

use std::time::Duration;

use bevy::math::Vec3;

use crate::prelude::PointNetwork;

impl PointNetwork {
    /// Moment of inertia along an axis.
    ///
    /// The axis is equivalent to a torque vector when normalized.
    ///
    /// ## Undefined Behavior
    /// If axis is not normalized, calculations will go wrong here!
    /// It is not normalized in this function for performance reasons.
    fn moment_of_inertia_along_axis(&self, axis: Vec3) -> f32 {
        let com = self.center_of_mass();
        self.points
            .iter()
            .map(|point| -> f32 { (point.pos - com).cross(axis).length_squared() * point.mass })
            .sum()
    }

    pub fn apply_torque_instant(&mut self, torque: Vec3) {
        if torque == Vec3::ZERO {
            return;
        }

        let com = self.center_of_mass();
        let strength = torque.length();
        let axis = torque.normalize();
        let moi = self.moment_of_inertia_along_axis(axis);

        for point in self.points.iter_mut() {
            let linear_dir = axis.cross(point.pos - com);
            point.vel += linear_dir * strength / moi;
        }
    }

    pub fn apply_torque_continuous(&mut self, torque: Vec3, delta_time: Duration) {
        self.apply_torque_instant(torque * delta_time.as_secs_f32());
    }
}
