//! # Object physics engine.
//!
//! Objects in Loot & Roam are physically comprised of a network of "points",
//! which are mobile points in 3D space, to which can be attached physical
//! volumes, visual point-attached renderers, and springs between points.

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

// [WIP] Please uncomment *only* implemented modules.
// pub mod volume;    // Volumes, their intersection, and volume/surface forces
// pub mod collision; // Advanced collision handling for objec

use bevy::{prelude::*, render::extract_component::ExtractComponent};
use itertools::iproduct;
use ultraviolet::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct PhysPoint {
    /// The position of this physics point in space.
    pub pos: Vec3,

    /// The velocity of this physics point.
    pub vel: Vec3,

    /// The mass of this physics point.
    pub mass: f32,
}

impl PhysPoint {
    /// Construct a new PhysPoint, setting only its position.
    ///
    /// Mass defaults to 1.0.
    pub fn from_pos(vec: Vec3) -> Self {
        Self {
            pos: vec,
            vel: Vec3::zero(),
            mass: 1.0,
        }
    }

    /// Construct a new PhysPoint, setting every field.
    pub fn new(pos: Vec3, vel: Vec3, mass: f32) -> Self {
        Self { pos, vel, mass }
    }

    /// Construct a new PhysPoint, with everything set to zero.
    pub fn zero() -> Self {
        Self::from_pos(Vec3::zero())
    }

    /// Sets this PhysPoint's position and returns itself.
    pub fn with_pos(&mut self, pos: Vec3) -> &mut Self {
        self.pos = pos;
        self
    }

    /// Sets this PhysPoint's velocity and returns itself.
    pub fn with_vel(&mut self, vel: Vec3) -> &mut Self {
        self.vel = vel;
        self
    }

    /// Sets this PhysPoint's mass and returns itself.
    pub fn with_mass(&mut self, mass: f32) -> &mut Self {
        self.mass = mass;
        self
    }

    /// Applies an instant force to this point (without applying delta time),
    /// and returns the point itself.
    pub fn apply_instant_force(&mut self, force: Vec3) -> &mut Self {
        self.vel += force / self.mass;
        self
    }
}

/// A network of physics points.
///
/// A component that must be in any physics-capable entity.
#[derive(Component)]
pub struct PointNetwork {
    pub points: Vec<PhysPoint>,
}

pub impl PointNetwork {
    /// Produces a SpringNetwork connected according to some criterion.
    pub fn make_connected_springs_whenever(
        &self,
        mode: SpringMode,
        predicate: Fn(&PhysPoint, &PhysPoint) -> bool,
    ) -> SpringNetwork {
        let springs: Vec<Spring> = iproduct!(
            self.points.iter().enumerate(),
            self.points.iter().enumerate()
        )
        .filter_map(|(point_1, point_2)| {
            if point_1.0 != point_2.0 && predicate(point_1.1, point_2.1) {
                Some(Spring {
                    points: (point_1.0, point_2.0),
                    rest_dist: (poin1_1.1.pos - point_2.1.pos).mag(),
                    mode,
                })
            } else {
                None
            }
        });

        SpringNetwork { springs }
    }

    /// Produces a SpringNetwork that is fully connected.
    pub fn make_fully_connected_springs(&self, mode: SpringMode) -> SpringNetwork {
        self.make_connected_springs_whenever(mode, || true)
    }

    /// Produces a SpringNetwork that connects points within a max radius.
    pub fn make_radially_connected_springs(&self, mode: SpringMode, max_rad: f32) -> SpringNetwork {
        let max_rad_sq = max_rad * max_rad;
        self.make_connected_springs_whenever(mode, |point_1, point_2| {
            (point_1.pos - point_2.pos).mag_sq() <= max_rad_sq
        })
    }
}

/// The system repsonsible for the inertia of physics points.
pub fn point_base_physics(time: Res<Time>, mut query_points: Query<(&mut PhysPoint)>) {
    let delta_secs = time.delta_seconds();

    for (point) in query_points.itert() {
        point.pos += point.vel * delta_secs;
    }
}

//---- Springs

/// The parameters for a normal-mode spring.
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
#[derive(Component)]
pub struct SpringNetwork {
    /// The list of springs in this network.
    pub springs: Vec<Spring>,
}

/// The system responsible for computing the spring system and its forces on points.
pub fn point_spring_forces(time: Res<Time>, mut query: Query<(&mut PhysPoint, &SpringNetwork)>) {
    let delta_secs = time.delta_seconds();

    for (points, springs) in query.iter() {
        for spring in springs.springs.iter() {
            let points: (&PhysPoint, &PhysPoint) = (
                &points.points[spring.points.0],
                &points.points[spring.points.1],
            );

            // [NOTE] All forces are relative to point A.
            // As such, they will be applied half to point A, half to point B
            // inverted.
            let relative = points.1.pos - points.0.pos;
            let unit_inward = relative.normalized();
            let dist = relative.mag();

            // If positive, dist must decrease (inward  force)
            // If negative, dist must increase (outward force)
            let dist_diff = dist - spring.rest_dist;

            match spring.mode {
                SpringMode::Instant => {
                    let offset = unit_inward * dist_diff;
                    let half_offset = offset / 2;

                    points.0.pos += half_offset;
                    points.1.pos -= half_offset;
                }

                SpringMode::Normal(force) => {
                    let force = unit_inward * dist_diff * force * delta_secs;
                    let half_force = force / 2;

                    points.0.apply_instant_force(half_force);
                    points.1.apply_instant_force(-half_force);
                }
            }
        }
    }
}
