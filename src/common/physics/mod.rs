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

// [TODO] Please uncomment *only* implemented modules.
// pub mod volume;    // Volumes, their intersection, and volume/surface forces
// pub mod collision; // Advanced collision handling for objec

use bevy::prelude::*;
use itertools::iproduct;

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
            vel: Vec3::ZERO,
            mass: 1.0,
        }
    }

    /// Construct a new PhysPoint, setting every field.
    pub fn new(pos: Vec3, vel: Vec3, mass: f32) -> Self {
        Self { pos, vel, mass }
    }

    /// Construct a new PhysPoint, with everything set to zero.
    pub fn zero() -> Self {
        Self::from_pos(Vec3::ZERO)
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

    /// Applies a continuous force to this point (taking delta time into account),
    /// and returns the point itself.
    pub fn apply_force_over_time(&mut self, force: Vec3, delta_secs: f32) -> &mut Self {
        let accel = force / self.mass;
        self.vel += accel * delta_secs;
        self.pos += 0.5 * accel * delta_secs.powi(2);
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

impl<Iter> From<Iter> for PointNetwork
where
    Iter: Iterator<Item = PhysPoint>,
{
    fn from(value: Iter) -> Self {
        Self {
            points: value.collect(),
        }
    }
}

impl PointNetwork {
    /// Produces a SpringNetwork connected according to some criterion.
    pub fn make_connected_springs_whenever<F>(
        &self,
        mode: SpringMode,
        predicate: F,
    ) -> SpringNetwork
    where
        F: Fn(&PhysPoint, &PhysPoint) -> bool,
    {
        let springs: Vec<Spring> = iproduct!(
            self.points.iter().enumerate(),
            self.points.iter().enumerate()
        )
        .filter_map(|(point_1, point_2)| {
            if point_1.0 != point_2.0 && predicate(point_1.1, point_2.1) {
                Some(Spring {
                    points: (point_1.0, point_2.0),
                    rest_dist: (point_1.1.pos - point_2.1.pos).length(),
                    mode,
                })
            } else {
                None
            }
        })
        .collect();

        SpringNetwork { springs }
    }

    /// Produces a SpringNetwork that is fully connected.
    pub fn make_fully_connected_springs(&self, mode: SpringMode) -> SpringNetwork {
        self.make_connected_springs_whenever(mode, |_, _| true)
    }

    /// Produces a SpringNetwork that connects points within a max radius.
    pub fn make_radially_connected_springs(&self, mode: SpringMode, max_rad: f32) -> SpringNetwork {
        let max_rad_sq = max_rad * max_rad;
        self.make_connected_springs_whenever(mode, |point_1, point_2| {
            (point_1.pos - point_2.pos).length_squared() <= max_rad_sq
        })
    }
}

/// The system repsonsible for the inertia of physics points.
pub fn point_base_physics(time: Res<Time>, mut query_points: Query<(&mut PointNetwork,)>) {
    let delta_secs = time.delta_secs();

    for (mut network,) in query_points.iter_mut() {
        for point in network.points.iter_mut() {
            point.pos += point.vel * delta_secs;
        }
    }
}

//---- Springs

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
#[derive(Component)]
pub struct SpringNetwork {
    /// The list of springs in this network.
    pub springs: Vec<Spring>,
}

/// The system responsible for computing the spring system and its forces on points.
pub fn point_spring_forces(time: Res<Time>, mut query: Query<(&mut PointNetwork, &SpringNetwork)>) {
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

/// This Bevy component applies gravity to a physics-enabled object.
///
/// Requires ]PointNetwork].
#[derive(Component)]
pub struct Gravity {
    /// The force of gravity, with direction and magnitude.
    ///
    /// Defaults to -10 in the Y axis.
    pub(crate) force: Vec3,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            force: Vec3::Y * -10.0,
        }
    }
}

/// The system responsible for gravity in the physics system.
pub fn gravity(time: Res<Time>, mut query: Query<(&mut PointNetwork, &Gravity)>) {
    for (mut points, gravity) in query.iter_mut() {
        for point in points.points.iter_mut() {
            point.vel += gravity.force * time.delta_secs();

            // Position change integration (approximating as that of a continuous linear acceleration)
            point.pos += 0.5 * gravity.force * time.delta_secs().powi(2);
        }
    }
}

pub fn apply_physics_systems(app: &mut App) {
    app.add_systems(Update, (gravity, point_base_physics, point_spring_forces));
}
