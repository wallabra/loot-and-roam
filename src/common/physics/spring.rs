//! # Spring physics
//!
//! Springs connect between physics points. To achieve this, the SpringNetwork
//! is a Bevy component, which only applies to entities which also share the
//! [PointNetwork] component.

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

use super::base::{PhysPoint, PointNetwork};

/// The parameters for a normal-mode spring.
#[derive(Debug, Clone, Copy)]
pub struct NormalSpring {
    /// The stiffness of the string.
    ///
    /// This is a linear scale on the force exerted on points to bring them
    /// either closer to or apart from each other, to converge the real
    /// distance towards the at-rest distance.
    pub stiffness: f32,
}

/// The spring mode.
///
/// Determines how a spring connects two points.
#[derive(Debug, Clone, Copy)]
pub enum SpringMode {
    /// Instant mode - points snap to the exact target distance.
    Instant,

    /// Normal mode - pushes the points closer to rest according to stiffness.
    Normal(NormalSpring),
}

/// A spring connecting two points.
#[derive(Debug, Clone)]
pub struct Spring {
    /// The index of points A and B into the PointNetwork.
    pub points: (usize, usize),

    /// The target/rest distance.
    pub rest_dist: f32,

    /// The spring mode.
    pub mode: SpringMode,
}

/// A spring network.
///
/// A component that must be used to link points together, regardless of how
/// spring-like their joints should actually be.
#[derive(Component, Clone, Default)]
pub struct SpringNetwork {
    /// The list of springs in this network.
    pub springs: Vec<Spring>,
}

/// The system responsible for computing the spring system and its forces on points.
fn point_spring_forces(time: Res<Time>, mut query: Query<(&mut PointNetwork, &SpringNetwork)>) {
    let delta_secs = time.delta_secs();

    for (mut points, springs) in query.iter_mut() {
        for spring in springs.springs.iter() {
            let point_data: (PhysPoint, PhysPoint) = (
                points.points[spring.points.0],
                points.points[spring.points.1],
            );

            // [NOTE] All forces are relative to point A.
            // As such, they will be applied half to point A, half to point B
            // inverted.
            let relative = point_data.1.pos - point_data.0.pos;
            let unit_inward = relative.normalize();
            let dist = relative.length();

            // If positive, dist must decrease (inward  force)
            // If negative, dist must increase (outward force)
            let dist_diff = dist - spring.rest_dist;

            match spring.mode {
                SpringMode::Instant => {
                    let offset = unit_inward * dist_diff;
                    let half_offset = offset * 0.5;

                    points.points[spring.points.0].pos += half_offset;
                    points.points[spring.points.1].pos -= half_offset;
                }

                SpringMode::Normal(mode) => {
                    let force = unit_inward * dist_diff * mode.stiffness;
                    let half_force = force * 0.5;

                    points.points[spring.points.0].apply_force_over_time(half_force, delta_secs);
                    points.points[spring.points.1].apply_force_over_time(-half_force, delta_secs);
                }
            }
        }
    }
}

/// Spring system plugin.
pub struct SpringForcesPlugin;

impl Plugin for SpringForcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (point_spring_forces,));
    }
}
