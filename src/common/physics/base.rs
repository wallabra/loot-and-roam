//! # Basic physics definitions and systems
//!
//! Physics points and their most basic systems (inertia and gravity) are
//! defined here.

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
use itertools::iproduct;

use super::spring::{Spring, SpringMode, SpringNetwork};

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
#[derive(Component, Clone, Default)]
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
    // [TODO] Find way to move spring network creation functions onto the spring module

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

/// The system responsible for the inertia of physics points.
pub fn point_base_physics(time: Res<Time>, mut query_points: Query<(&mut PointNetwork,)>) {
    let delta_secs = time.delta_secs();

    for (mut network,) in query_points.iter_mut() {
        for point in network.points.iter_mut() {
            point.pos += point.vel * delta_secs;
        }
    }
}

/// Use this component on a child entity to attach it to a physics point of its parent.
///
/// The parent must have a [PointNetwork] component.
#[derive(Component)]
pub struct PointAttach {
    /// The index of the physics point on the parent's [PointNetwork].
    pub point_idx: usize,
}

// Always runs after point_base_physics.
fn point_attach_snap(
    mut query_child: Query<(&ChildOf, &mut Transform, &PointAttach)>,
    query_parent: Query<(&PointNetwork, &GlobalTransform, &Transform), Without<PointAttach>>,
) {
    for (child_of, mut transform, attachment) in query_child.iter_mut() {
        let (parent_points, parent_global_transform, parent_transform) =
            query_parent.get(child_of.parent()).unwrap();

        assert!(attachment.point_idx < parent_points.points.len());

        transform.translation =
            parent_points.points[attachment.point_idx].pos - parent_global_transform.translation();
        transform.rotate_around(Vec3::ZERO, parent_transform.rotation.inverse());
    }
}
