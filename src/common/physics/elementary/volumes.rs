use core::f32;

use enum_dispatch::enum_dispatch;
use ultraviolet::Vec3;

/**
 * A spherical volume primitive.
 */
 #[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub radius: f32,
}

/**
 * A cylinder volume primitive.
 */
#[derive(Debug, Clone, Copy)]
pub struct Cylinder {
    pub height: f32,
    pub radius: f32,
}

/**
 * A volume primitive.
 *
 * Those are internally used in the physics engine, often composed into
 * volume assemblies, to shape interactions between game objects, such as
 * collision, buoyancy, drag & friction, etc.
 */
#[enum_dispatch]
pub trait VolumeImpl {
    /// Tests whether a point is inside this primitive.
    fn point_in_primitive(&self, center: Vec3, point: Vec3) -> bool;

    /// Calculates the volume of this primitive.
    fn volume(&self) -> f32;

    /// Calculates the total surface area of this primitive.
    fn surface_area(&self) -> f32;
}

#[enum_dispatch(Volume)]
pub enum Volume {
    Sphere,
    Cylinder,
}

impl VolumeImpl for Sphere {
    fn point_in_primitive(&self, center: Vec3, point: Vec3) -> bool {
        (point - center).mag_sq() < self.radius * self.radius
    }

    fn volume(&self) -> f32 {
        self.radius * self.radius * self.radius * f32::consts::FRAC_PI_4 * 3.0
    }

    fn surface_area(&self) -> f32 {
        self.radius * self.radius * f32::consts::PI * 4.0
    }
}

impl VolumeImpl for Cylinder {
    fn point_in_primitive(&self, center: Vec3, point: Vec3) -> bool {
        let off = point - center;
        off.xy().mag_sq() < self.radius * self.radius && off.z.abs() < self.height
    }

    fn volume(&self) -> f32 {
        self.radius * self.radius * self.height * f32::consts::PI
    }

    fn surface_area(&self) -> f32 {
        self.radius * (self.radius + self.height) * f32::consts::PI * 2.0
    }
}

#[derive(Debug, Clone)]
pub struct VolumePrimitive {
    pub volume: Volume,
    pub on_point: usize,
}
