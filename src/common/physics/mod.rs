use std::iter::repeat;
use std::ops::{Index, Range};

use elementary::volumes::VolumePrimitive;
use elementary::{Point, Spring};
use itertools::izip;
use slotmap::{new_key_type, HopSlotMap, Key, SlotMap};
use smallvec::SmallVec;
use ultraviolet::Vec3;

// NOTE: separate collider system from physics system
// (use rigid/softbody system to compose geom primitives, use physics sys to
//  give them inertia individually, use collision sys to find and handle
//  collisions)
pub mod collision;
pub mod elementary;
pub mod subsystems;

new_key_type! {
    pub struct PointKey;
    pub struct VelocityKey;
    pub struct SpringKey;
    pub struct ObjectRegionHandle;
}

/**
 * Reference to a contiguuos region of the PhysicsRegistry, associated with a
 * single object.
 */
#[derive(Clone, Debug)]
pub struct ObjectRegion {
    point_range: Range<usize>,
    spring_range: Range<usize>,
    volume_range: Range<usize>,
}

/**
 * Contains all physics elements being simulated.
 */
#[derive(Clone, Debug, Default)]
pub struct PhysicsRegistry {
    points: Vec<Point>,
    velocities: Vec<Vec3>,
    springs: Vec<Spring>,
    volumes: Vec<VolumePrimitive>,
    regions: SlotMap<ObjectRegionHandle, ObjectRegion>,
}

impl PhysicsRegistry {
    pub fn new(
        points: Vec<Point>,
        springs: Vec<Spring>,
        velocities: Option<Vec<Vec3>>,
        regions: Option<SlotMap<SpringObjectHandle, ObjectRegion>>,
    ) -> Self {
        let mut velocities = velocities.unwrap_or(vec![]);
        velocities.extend(repeat(Vec3::zero()).take(points.len() - velocities.len()));

        Self {
            points,
            springs,
            velocities,
            regions: regions.unwrap_or_default(),
        }
    }

    /*
    /**
     * Add a whole object's worth of points and springs to the registry.
     */
    pub fn add_object(&mut self, object: ObjectSpringNetwork) -> SpringObjectHandle {
        let region = ObjectRegion {
            point_region: (self.points.len(), object.points.len()),
            spring_region: (self.springs.len(), object.springs.len()),
        };

        self.points.extend_from_slice(&object.points);
        self.springs.extend_from_slice(&object.springs);
        self.velocities
            .extend(repeat(object.initial_velocity.unwrap_or_default()).take(object.points.len()));
        self.regions.insert(region)
    }
    */

    pub fn remove_object(&mut self, handle: SpringObjectHandle) {
        let region = self.regions[handle];
        let (point_region_at, point_region_len) = region.point_region;
        let (spring_region_at, spring_region_len) = region.spring_region;

        // Remove managed points, velocities and springs
        self.points
            .drain(point_region_at..point_region_at + point_region_len);
        self.velocities.drain(..point_region_at + point_region_len);
        self.springs
            .drain(spring_region_at..spring_region_at + spring_region_len);

        // Update regions
        self.regions.iter_mut().for_each(|(_key, reg)| {
            if reg.point_region.0 < point_region_at + point_region_len
                || reg.spring_region.0 < spring_region_at + spring_region_len
            {
                return;
            }
            reg.point_region.0 -= point_region_len;
            reg.spring_region.0 -= spring_region_len;
        });

        self.regions.remove(handle);
    }

    pub fn set_points(&mut self, new_points: &[Point]) {
        self.points.copy_from_slice(new_points);
    }

    pub fn apply_point_forces(&mut self, forces: &[Vec3], delta_time: f32) {
        izip!(
            &mut self.velocities,
            forces,
            self.points.iter().map(|p| p.mass)
        )
        .for_each(|(vel, force, mass)| *vel += *force * delta_time / mass);
    }

    pub fn get_spring_forces(&self) -> Vec<Vec3> {
        struct SingleForce {
            which_point: usize,
            force: Vec3,
        }

        let forces = self
            .springs
            .iter()
            .map(|spring| -> [SingleForce; 2] {
                let point_a = self.points[spring.which_point_a];
                let point_b = self.points[spring.which_point_b];
                let total_mass = point_a.mass + point_b.mass;
                let recip_mass_ratio_a = total_mass / point_a.mass;
                let recip_mass_ratio_b = total_mass / point_b.mass;
                let offs = point_b.pos - point_a.pos;
                let offs_norm = offs.normalized();
                let mag = offs.mag();
                let mag_offs = spring.equilibrium_length - mag;
                let dist_force = mag_offs * spring.stiffness;
                let force_a_b = offs_norm * dist_force;
                [
                    SingleForce {
                        which_point: spring.which_point_a,
                        force: force_a_b * recip_mass_ratio_a,
                    },
                    SingleForce {
                        which_point: spring.which_point_b,
                        force: -force_a_b * recip_mass_ratio_b,
                    },
                ]
            })
            .flatten()
            .collect::<Vec<_>>();

        (0..self.points.len())
            .map(|i| {
                forces
                    .iter()
                    .filter_map(|f| {
                        if f.which_point == i {
                            Some(f.force)
                        } else {
                            None
                        }
                    })
                    .sum()
            })
            .collect::<_>()
    }
}
