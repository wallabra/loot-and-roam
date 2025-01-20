use std::iter::repeat;

use itertools::izip;
use slotmap::{new_key_type, SlotMap};
use ultraviolet::Vec3;

new_key_type! {
    pub struct SpringTypeHandle;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SpringParams {
    pub(crate) distance: f32,
    pub(crate) strength: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Spring {
    pub(crate) which_point_a: usize,
    pub(crate) which_point_b: usize,
    pub(crate) which_param: SpringTypeHandle, // no benefit in being u8, because of struct alignment
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub(crate) pos: Vec3,
    pub(crate) mass: f32,
}

#[derive(Clone, Debug, Default)]
pub struct ObjectSpringNetwork {
    pub points: Vec<Point>,
    pub springs: Vec<Spring>,
    pub initial_velocity: Option<Vec3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ObjectRegion {
    pub point_region: (usize, usize),
    pub spring_region: (usize, usize),
}

new_key_type! {
    pub struct SpringObjectHandle;
}

#[derive(Clone, Debug, Default)]
pub struct SpringSystem {
    spring_types: SlotMap<SpringTypeHandle, SpringParams>,
    points: Vec<Point>,
    springs: Vec<Spring>,
    velocities: Vec<Vec3>,
    regions: SlotMap<SpringObjectHandle, ObjectRegion>,
}

impl SpringSystem {
    pub fn new(
        spring_types: SlotMap<SpringTypeHandle, SpringParams>,
        points: Vec<Point>,
        springs: Vec<Spring>,
        velocities: Option<Vec<Vec3>>,
        regions: Option<SlotMap<SpringObjectHandle, ObjectRegion>>
    ) -> Self {
        let mut velocities = velocities.unwrap_or(vec![]);
        velocities.extend(repeat(Vec3::zero()).take(points.len() - velocities.len()));

        Self {
            spring_types,
            points,
            springs,
            velocities,
            regions: regions.unwrap_or_default(),
        }
    }
    
    pub fn add_object(&mut self, object: ObjectSpringNetwork) -> SpringObjectHandle {
        let region = ObjectRegion {
            point_region: (self.points.len(), object.points.len()),
            spring_region: (self.springs.len(), object.springs.len()),
        };
        
        self.points.extend_from_slice(&object.points);
        self.springs.extend_from_slice(&object.springs);
        self.velocities.extend(repeat(object.initial_velocity.unwrap_or_default()).take(object.points.len()));
        self.regions.insert(region)
    }
    
    pub fn remove_object(&mut self, handle: SpringObjectHandle) {
        let region = self.regions[handle];
        let (point_region_at, point_region_len) = region.point_region;
        let (spring_region_at, spring_region_len) = region.spring_region;
        
        // Remove managed points, velocities and springs
        self.points.drain(point_region_at..point_region_at + point_region_len);
        self.velocities.drain(..point_region_at + point_region_len);
        self.springs.drain(spring_region_at..spring_region_at + spring_region_len);
        
        // Update regions
        self.regions.iter_mut().for_each(|(_key, reg)| {
            if reg.point_region.0 < point_region_at + point_region_len || reg.spring_region.0 < spring_region_at + spring_region_len {
                return;
            }
            reg.point_region.0 -= point_region_len;
            reg.spring_region.0 -= spring_region_len;
        });
        
        self.regions.remove(handle);
    }

    pub fn register_spring_type(&mut self, spring_type: SpringParams) -> SpringTypeHandle {
        self.spring_types.insert(spring_type)
    }

    pub fn set_points(&mut self, new_points: &[Point]) {
        self.points.copy_from_slice(new_points);
    }

    pub fn tick(&mut self, delta_time: f32) {
        self.apply_forces(&self.get_forces(), delta_time);
    }

    pub fn apply_forces(&mut self, forces: &[Vec3], delta_time: f32) {
        izip!(
            &mut self.velocities,
            forces,
            self.points.iter().map(|p| p.mass)
        )
        .for_each(|(vel, force, mass)| *vel += *force * delta_time / mass);
    }

    pub fn get_forces(&self) -> Vec<Vec3> {
        struct SingleForce {
            which_point: usize,
            force: Vec3,
        }

        let forces = self
            .springs
            .iter()
            .map(|spring| -> [SingleForce; 2] {
                let param = self.spring_types[spring.which_param];
                let point_a = self.points[spring.which_point_a];
                let point_b = self.points[spring.which_point_b];
                let total_mass = point_a.mass + point_b.mass;
                let recip_mass_ratio_a = total_mass / point_a.mass;
                let recip_mass_ratio_b = total_mass / point_b.mass;
                let offs = point_b.pos - point_a.pos;
                let offs_norm = offs.normalized();
                let mag = offs.mag();
                let mag_offs = param.distance - mag;
                let dist_force = mag_offs * param.strength;
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
