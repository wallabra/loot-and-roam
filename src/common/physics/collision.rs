use ultraviolet as uv;

#[derive(Debug, Clone, Copy)]
pub struct CollisionInfo {
    shape_offset: uv::Vec3,
    pub location: uv::Vec3,
    pub normal: uv::Vec3,
}

impl CollisionInfo {
    fn invert(self) -> Self {
        Self {
            shape_offset: self.shape_offset * -1.0,
            location: self.location - self.shape_offset,
            normal: self.normal * -1.0,
        }
    }
}

pub trait Collision<Other> {
    fn detect(&self, other: &Other, offset: uv::Vec3) -> Option<CollisionInfo>;
}

pub struct PointCollision;

impl Collision<PointCollision> for PointCollision {
    fn detect(&self, _other: &PointCollision, _offset: uv::Vec3) -> Option<CollisionInfo> {
        None
    }
}

impl Collision<PointCollision> for Sphere {
    fn detect(&self, _other: &PointCollision, offset: uv::Vec3) -> Option<CollisionInfo> {
        let dist = offset.mag();
        if dist > self.radius {
            None
        } else {
            Some(CollisionInfo {
                shape_offset: offset,
                location: offset,
                normal: offset / dist,
            })
        }
    }
}

impl Collision<PointCollision> for Cylinder {
    fn detect(&self, _other: &PointCollision, offset: uv::Vec3) -> Option<CollisionInfo> {
        let offset2 = offset.xy();
        let offsetz = offset.z;
        let dist2 = offset2.mag();

        if dist2 > self.radius || offsetz.abs() > self.height {
            None
        } else {
            Some(CollisionInfo {
                shape_offset: offset,
                location: offset,
                normal: offset / offset.mag(),
            })
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cylinder {
    pub radius: f32,
    pub height: f32,
}

impl Collision<Cylinder> for Cylinder {
    fn detect(&self, other: &Cylinder, offset: uv::Vec3) -> Option<CollisionInfo> {
        // find closest point in cylinder
        let xyoff = offset.xy();
        let xyclosest = xyoff.normalized() * xyoff.mag().min(self.radius);
        let zclosest = offset.z.clamp(-self.height, self.height);
        let closest = xyclosest.xyz() + uv::Vec3::unit_z() * zclosest;

        other
            .detect(&PointCollision, closest - offset)
            .map(|detection| detection.invert())
    }
}

impl Collision<Sphere> for Sphere {
    fn detect(&self, other: &Sphere, offset: uv::Vec3) -> Option<CollisionInfo> {
        let dist = offset.mag();
        if dist > self.radius + other.radius {
            None
        } else {
            Some(CollisionInfo {
                shape_offset: offset,
                location: offset / dist * self.radius,
                normal: offset / dist,
            })
        }
    }
}

impl Collision<Sphere> for Cylinder {
    fn detect(&self, other: &Sphere, offset: uv::Vec3) -> Option<CollisionInfo> {
        // find closest point in cylinder
        let xyoff = offset.xy();
        let xyclosest = xyoff.normalized() * xyoff.mag().min(self.radius);
        let zclosest = offset.z.clamp(-self.height, self.height);
        let closest = xyclosest.xyz() + uv::Vec3::unit_z() * zclosest;

        other
            .detect(&PointCollision, closest - offset)
            .map(|detection| detection.invert())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub radius: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum PrimitiveShape {
    Cylinder(Cylinder),
    Sphere(Sphere),
}

impl<T> Collision<PrimitiveShape> for T
where
    T: Collision<Sphere> + Collision<Cylinder>,
{
    fn detect(&self, other: &PrimitiveShape, offset: uv::Vec3) -> Option<CollisionInfo> {
        match other {
            PrimitiveShape::Cylinder(cyl) => self.detect(cyl, offset),
            PrimitiveShape::Sphere(sph) => self.detect(sph, offset),
        }
    }
}

impl Collision<PrimitiveShape> for PrimitiveShape {
    fn detect(&self, other: &PrimitiveShape, offset: uv::Vec3) -> Option<CollisionInfo> {
        match self {
            PrimitiveShape::Cylinder(cyl) => cyl.detect(other, offset),
            PrimitiveShape::Sphere(sph) => other
                .detect(&PrimitiveShape::Sphere(*sph), -offset)
                .map(|res| res.invert()),
        }
    }
}

pub struct PrimitiveCollider {
    offset: uv::Vec3,
    shape: PrimitiveShape,
}

impl Collision<PrimitiveCollider> for PrimitiveCollider {
    fn detect(&self, other: &PrimitiveCollider, offset: uv::Vec3) -> Option<CollisionInfo> {
        self.shape.detect(&other.shape, offset + other.offset)
    }
}

pub struct CompositeCollider {
    colliders: Vec<PrimitiveCollider>,
}

impl Collision<PrimitiveCollider> for CompositeCollider {
    fn detect(&self, other: &PrimitiveCollider, offset: uv::Vec3) -> Option<CollisionInfo> {
        self.colliders
            .iter()
            .map(|collider| collider.detect(other, offset))
            .find(|a| a.is_some())
            .unwrap_or(None)
    }
}

impl Collision<CompositeCollider> for CompositeCollider {
    fn detect(&self, other: &CompositeCollider, offset: uv::Vec3) -> Option<CollisionInfo> {
        self.colliders
            .iter()
            .map(|collider| other.detect(collider, -offset))
            .find(|a| a.is_some())
            .unwrap_or(None)
    }
}

#[cfg(test)]
mod tests {
    use assertables::{assert_none, assert_some};
    use ultraviolet as uv;

    use super::Collision;
    use super::Cylinder;
    use super::Sphere;

    #[test]
    fn test_sphere_sphere_intersection() {
        let sphere_1 = Sphere { radius: 5.0 };
        let sphere_2 = Sphere { radius: 3.0 };

        assert_some!(sphere_1.detect(&sphere_2, uv::Vec3::new(7.0, 0.0, 0.0)));
        assert_none!(sphere_1.detect(&sphere_2, uv::Vec3::new(9.0, 0.0, 0.0)));
        assert_none!(sphere_1.detect(&sphere_2, uv::Vec3::new(7.0, 7.0, 0.0)));
        assert_none!(sphere_1.detect(&sphere_2, uv::Vec3::new(7.0, 7.0, 7.0)));
    }

    #[test]
    fn test_cylinder_cylinder_intersection() {
        let cylinder_1 = Cylinder {
            radius: 5.0,
            height: 2.0,
        };
        let cylinder_2 = Cylinder {
            radius: 3.0,
            height: 2.0,
        };

        assert_some!(cylinder_1.detect(&cylinder_2, uv::Vec3::new(7.0, 0.0, 0.0)));
        assert_some!(cylinder_1.detect(&cylinder_2, uv::Vec3::new(7.0, 0.0, 1.0)));
        assert_none!(cylinder_1.detect(&cylinder_2, uv::Vec3::new(7.0, 0.0, 5.0)));
        assert_none!(cylinder_1.detect(&cylinder_2, uv::Vec3::new(9.0, 0.0, 0.0)));
        assert_none!(cylinder_1.detect(&cylinder_2, uv::Vec3::new(7.0, 7.0, 0.0)));
        assert_some!(cylinder_1.detect(&cylinder_2, uv::Vec3::new(3.0, 3.0, -2.0)));
    }

    #[test]
    fn test_cylinder_sphere_intersection() {
        let cylinder = Cylinder {
            radius: 5.0,
            height: 2.0,
        };
        let sphere = Sphere { radius: 3.00 };

        assert_some!(cylinder.detect(&sphere, uv::Vec3::new(7.0, 0.0, 0.0)));
        assert_some!(cylinder.detect(&sphere, uv::Vec3::new(7.0, 0.0, 1.0)));
        assert_some!(cylinder.detect(&sphere, uv::Vec3::new(8.0, 0.0, 1.0)));
        assert_none!(cylinder.detect(&sphere, uv::Vec3::new(8.0, 0.0, 3.0)));
        assert_none!(cylinder.detect(&sphere, uv::Vec3::new(7.0, 0.0, 6.0)));
        assert_none!(cylinder.detect(&sphere, uv::Vec3::new(9.0, 0.0, 0.0)));
        assert_none!(cylinder.detect(&sphere, uv::Vec3::new(7.0, 7.0, 0.0)));
        assert_some!(cylinder.detect(&sphere, uv::Vec3::new(3.0, 3.0, -2.0)));
    }
}
