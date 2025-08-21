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

    /// Applies an instant rotational force (angular impulse).
    pub fn apply_angular_impulse(&mut self, angular_impulse: Vec3) {
        if angular_impulse == Vec3::ZERO {
            return;
        }

        let center_of_mass = self.center_of_mass();
        let impulse_strength = angular_impulse.length();
        let impulse_axis = angular_impulse.normalize();
        let moment_of_inertia = self.moment_of_inertia_along_axis(impulse_axis);

        for point in self.points.iter_mut() {
            // -- physics note --
            // impulse_axis is a unit vector
            // multiplying it by the square of distance from rotational axis crossing COM makes it dist^2
            // multiplying it by the 'impulse strength' (magnitude of angular_impulse) makes it dist^4*mass
            //   (this is because unit vector has no mass or vel. information, simply giving a magnitude its direction)
            // dividing it by moment_of_inertia (which is dist^2*mass) makes it dist^2
            // delta velocities within a single tick are applied directly to velocity, therefore lack a time component

            let linear_delta_velocity =
                impulse_axis.cross((point.pos - center_of_mass).powf(2.0)) * impulse_strength;

            point.vel += linear_delta_velocity / moment_of_inertia;
        }
    }

    /// Applies a rotational force (torque) spread over a tick's duration.
    pub fn apply_torque(&mut self, torque: Vec3, delta_time: Duration) {
        self.apply_angular_impulse(torque * delta_time.as_secs_f32());
    }
}
