use vector3d::Vector3d;

pub struct CollisionInfo {
    shape_offset: Vector3d<f64>,
    pub location: Vector3d<f64>,
    pub normal: Vector3d<f64>,
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
    fn detect(&self, other: &Other, offset: Vector3d<f64>) -> Option<CollisionInfo>;
}

impl<A, B> Collision<A> for B
where
    A: Collision<B>,
{
    default fn detect(&self, other: &A, offset: Vector3d<f64>) -> Option<CollisionInfo> {
        other
            .detect(self, offset * -1.0)
            .map(|detection| detection.invert())
    }
}

pub struct PointCollision;

impl Collision<PointCollision> for PointCollision {
    fn detect(&self, _other: &PointCollision, _offset: Vector3d<f64>) -> Option<CollisionInfo> {
        None
    }
}

impl Collision<PointCollision> for Sphere {
    fn detect(&self, _other: &PointCollision, offset: Vector3d<f64>) -> Option<CollisionInfo> {
        let dist = offset.dot(offset).sqrt();
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
    fn detect(&self, _other: &PointCollision, offset: Vector3d<f64>) -> Option<CollisionInfo> {
        let offset2 = Vector3d::new(offset.x, offset.y, 0.0);
        let offsetz = offset.z;
        let dist2 = offset2.dot(offset2).sqrt();

        if dist2 > self.radius || offsetz.abs() > self.height {
            None
        } else {
            Some(CollisionInfo {
                shape_offset: offset,
                location: offset,
                normal: offset / offset.dot(offset).sqrt(),
            })
        }
    }
}

pub struct Cylinder {
    pub radius: f64,
    pub height: f64,
}

impl Collision<Cylinder> for Cylinder {
    fn detect(&self, other: &Cylinder, offset: Vector3d<f64>) -> Option<CollisionInfo> {
        // find closest point in cylinder
        let xyoff = Vector3d::new(offset.x, offset.y, 0.0);
        let xyclosest = Vector3d::new(1.0, 1.0, 0.0) * xyoff.dot(xyoff).sqrt().min(self.radius);
        let zclosest = offset.z.clamp(-self.height, self.height);
        let closest = xyclosest + Vector3d::<f64>::new(0.0, 0.0, 1.0) * zclosest;

        other
            .detect(&PointCollision, closest - offset)
            .map(|detection| detection.invert())
    }
}

impl Collision<Sphere> for Sphere {
    fn detect(&self, other: &Sphere, offset: Vector3d<f64>) -> Option<CollisionInfo> {
        let dist = offset.dot(offset).sqrt();
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
    fn detect(&self, other: &Sphere, offset: Vector3d<f64>) -> Option<CollisionInfo> {
        // find closest point in cylinder
        let xyoff = Vector3d::new(offset.x, offset.y, 0.0);
        let xyclosest = Vector3d::new(1.0, 1.0, 0.0) * xyoff.dot(xyoff).sqrt().min(self.radius);
        let zclosest = offset.z.clamp(-self.height, self.height);
        let closest = xyclosest + Vector3d::<f64>::new(0.0, 0.0, 1.0) * zclosest;

        other
            .detect(&PointCollision, closest - offset)
            .map(|detection| detection.invert())
    }
}

pub struct Sphere {
    pub radius: f64,
}

pub enum PrimitiveShape {
    Cylinder(Cylinder),
    Sphere(Sphere),
}

impl<T> Collision<PrimitiveShape> for T
where
    T: Collision<Sphere> + Collision<Cylinder>,
{
    fn detect(&self, other: &PrimitiveShape, offset: Vector3d<f64>) -> Option<CollisionInfo> {
        match other {
            PrimitiveShape::Cylinder(cyl) => self.detect(cyl, offset),
            PrimitiveShape::Sphere(sph) => self.detect(sph, offset),
        }
    }
}

impl Collision<PrimitiveShape> for PrimitiveShape {
    fn detect(&self, other: &PrimitiveShape, offset: Vector3d<f64>) -> Option<CollisionInfo> {
        match self {
            PrimitiveShape::Cylinder(cyl) => cyl.detect(other, offset),
            PrimitiveShape::Sphere(sph) => sph.detect(other, offset),
        }
    }
}

pub struct PrimitiveCollider {
    offset: Vector3d<f64>,
    shape: PrimitiveShape,
}

impl Collision<PrimitiveCollider> for PrimitiveCollider {
    fn detect(&self, other: &PrimitiveCollider, offset: Vector3d<f64>) -> Option<CollisionInfo> {
        self.shape.detect(&other.shape, offset + other.offset)
    }
}

pub struct CompositeCollider {
    colliders: Vec<PrimitiveCollider>,
}

pub struct PhysicsContext {}

pub struct PhysicsObject {
    pos: Vector3d<f64>,
    vel: Vector3d<f64>,
}

impl PhysicsObject {}
