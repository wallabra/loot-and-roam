//! # Physics volumes
// [TODO] Better doc module description

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

use std::ops::Range;

use bevy::prelude::*;
use enum_dispatch::enum_dispatch;
use range_ext::intersect::Intersect;

use super::base::{PhysPoint, PointNetwork};

/// Axis-aligned bounding box.
///
/// Used for quick preliminary intersection checks.
#[derive(Debug, Clone, Default)]
pub struct AABB {
    /// Spans on the X, Y and Z axes, respectively.
    pub spans: [Range<f32>; 3],
}

fn span_union(span_a: &Range<f32>, span_b: &Range<f32>) -> Range<f32> {
    span_a.start.min(span_b.start)..span_a.end.min(span_b.end)
}

impl AABB {
    /// Initialize an AABB with three ranges, one for each axis: X, Y and Z.
    pub fn new(range_x: Range<f32>, range_y: Range<f32>, range_z: Range<f32>) -> Self {
        Self {
            spans: [range_x, range_y, range_z],
        }
    }

    /// Check whether two AABBs intersect.
    pub fn check(&self, other: AABB) -> bool {
        self.spans
            .iter()
            .zip(other.spans.iter())
            .all(|(span_a, span_b)| span_a.does_intersect(span_b))
    }

    /// Returns a new AABB which contains both input AABBs in it.
    pub fn union(self, other: AABB) -> Self {
        Self {
            spans: std::array::from_fn(|i| span_union(&self.spans[i], &other.spans[i])),
        }
    }

    /// Return a copy of this AABB, fully translated along a 3D vector.
    pub fn translate(self, translation: Vec3) -> Self {
        let coords = [translation.x, translation.y, translation.z];

        Self {
            spans: std::array::from_fn(|i| {
                self.spans[i].start + coords[i]..self.spans[i].end + coords[i]
            }),
        }
    }
}

/// Tiny value used for gradient sampling.
///
/// This is used in the default SDF-based normal implementation,
/// [VolumeInfo::normal].
const EPSILON: f32 = 0.000001;

/// Interface for implementing arbitrary volume for collision detection.
///
/// All methods take the volume's origin to be at (0,0,0). If you want to use a
/// different origin point, you have to first translate the arguments you want
/// to use into the volume's local space.
///
/// This interface is best geared for convex geometry; the same assumption is
/// made in the collision algorithm used here.
#[enum_dispatch]
pub trait VolumeInfo {
    /// Returns the closest point to the refenrece point within the set of
    /// points in this volume.
    ///
    /// This assumes the volume's origin at (0,0,0).
    fn closest_point_to(&self, reference: Vec3) -> Vec3;

    /// Returns the signed distance field for this volume at pos.
    ///
    /// 'Signed distance' means that the return value is positive when pos is
    /// *uotside* the volume ("positive" distance), and negative when pos is
    /// *inside* the volume ("negative" distance).
    ///  
    /// This assumes the volume's origin at (0,0,0).
    fn sdf(&self, pos: Vec3) -> f32;

    /// Returns the outward normal vector at pos.
    ///
    /// The default implementation samples sdf() four times - once at pos, and
    /// once for each axis.
    ///
    /// This assumes the volume's origin at (0,0,0).
    ///
    /// If pos is at origin, returns (0,1,0) as a default, to avoid undefined
    /// behavior or crashes.
    fn normal(&self, pos: Vec3) -> Vec3 {
        let sdf_at_point = self.sdf(pos);

        [Vec3::X, Vec3::Y, Vec3::Z]
            .into_iter()
            .map(|unit_vec| unit_vec * (self.sdf(pos + unit_vec * EPSILON) - sdf_at_point))
            .reduce(|v1, v2| v1 + v2)
            .unwrap_or(Vec3::Y)
            .normalize()
    }

    /// Return an AABB that wraps around this volume.
    ///
    /// Necessary for quick collision checks, before using the closest-point
    /// based algorithm.
    ///
    /// This assumes the volume's origin at (0,0,0).
    fn aabb(&self) -> AABB;

    /// Returns whether this point is inside this geometry.
    ///
    /// The default implementation checks whether the SDF at that point is
    /// negative; thsi should work fine for most use cases. For more info, see
    /// [VolumeInfo::sdf].
    fn point_is_within(&self, point: Vec3) -> bool {
        self.sdf(point) < 0.0
    }
}

/// Basic information on a detected collision.
pub struct CollisionInfo {
    /// The collision point.
    ///
    /// It isn't a surface point; it may be deeper into the surface depending
    /// on the intersection depth. Between two VolumeInfos, this is implemented
    /// as the average of both surface points.
    pub pos: Vec3,

    /// The normal of the collision.
    pub normal: Vec3,
}

/// Implmeent for objects that can have collision with volumes tested.
pub trait VolumeCollision {
    /// Returns whether this collides with a volume at offset.
    fn collides_with<T: VolumeInfo>(&self, volume: &T, offset: Vec3) -> bool {
        self.collision(volume, offset).is_some()
    }

    /// Returns the collision point.
    ///
    /// It isn't a surface point; it may be deeper into the surface depending
    /// on the intersection depth. Between two VolumeInfos, this is implemented
    /// as the average of both surface points.
    fn collision_point<T: VolumeInfo>(&self, volume: &T, offset: Vec3) -> Option<Vec3> {
        self.collision(volume, offset).map(|info| info.pos)
    }

    /// Returns the collision normal.
    fn collision_normal<T: VolumeInfo>(&self, volume: &T, offset: Vec3) -> Option<Vec3> {
        self.collision(volume, offset).map(|info| info.normal)
    }

    /// Performs the collision test algorithm and returns its results.
    ///
    /// If no collision is found, returns None.
    fn collision<T: VolumeInfo>(&self, volume: &T, offset: Vec3) -> Option<CollisionInfo>;
}

impl<V: VolumeInfo> VolumeCollision for V {
    /// Average of closest points collision algorithm.
    ///
    /// This algorithm only works with convex volumes.
    ///
    /// For non-convex geometries, approximate them with multiple volumes!
    /// That is the point of the [VolumeCollection] API!
    fn collision<T: VolumeInfo>(&self, volume: &T, offset: Vec3) -> Option<CollisionInfo> {
        let average_point =
            (self.closest_point_to(offset) + offset + volume.closest_point_to(-offset)) / 2.0;

        if self.point_is_within(average_point) && volume.point_is_within(average_point) {
            Some(CollisionInfo {
                pos: average_point,
                normal: self.normal(average_point),
            })
        } else {
            None
        }
    }
}

/// A Sphere-based volume.
#[derive(Debug, Clone, Copy, Default)]
pub struct SphereDef {
    /// The radius of this sphere, centered at its origin.
    pub radius: f32,
}

impl SphereDef {
    /// Return a new SphereDef, with a specified radius.
    ///
    /// The origin is assumed to be (0,0,0).
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl VolumeInfo for SphereDef {
    fn closest_point_to(&self, reference: Vec3) -> Vec3 {
        reference.clamp_length_max(self.radius)
    }

    fn sdf(&self, pos: Vec3) -> f32 {
        pos.length() - self.radius
    }

    fn normal(&self, pos: Vec3) -> Vec3 {
        pos.normalize()
    }

    fn aabb(&self) -> AABB {
        AABB::new(
            -self.radius..self.radius,
            -self.radius..self.radius,
            -self.radius..self.radius,
        )
    }
}

/// A volume definition.
///
/// All volume definitions are presumed to be at (0,0,0); see [VolumeInfo]
/// for details on this.
#[derive(Clone, Copy, Debug)]
#[enum_dispatch(VolumeInfo)]
pub enum VolumeType {
    Sphere(SphereDef),
}

impl Default for VolumeType {
    fn default() -> Self {
        Self::Sphere(SphereDef::new(0.5))
    }
}

/// A physics volume, attached to a physics point.
#[derive(Clone, Copy, Debug)]
pub struct PhysicsVolume {
    /// The physics point this volume should be attached to.
    pub point_idx: usize,

    /// The type of volume.
    ///
    /// Currently, only Spheres are implemented.
    // [NOTE] The above line may have to be updated in the future :)
    pub volume_type: VolumeType,
}

/// ECS component with a list of physics-point-attached volumes.
#[derive(Component)]
pub struct VolumeCollection {
    /// The physics volumes on this collection.
    ///
    /// Each of them is linked to a physics point by its point_idx member.
    /// The collision handler system uses this to define each volume's
    /// position at runtime, when doing collision checks.
    pub volumes: Vec<PhysicsVolume>,
}

/// Strategies for creating new volumes at every point on a PointNetwork.
///
/// Used in the VolumeCollection's PointNetwork-based constructors.
pub trait VolumeSpawner {
    /// Produces a VolumeType to be used at this point.
    fn volume_type_at(&self, point: &PhysPoint, point_idx: usize) -> VolumeType;
}

/// Clones a Volume at any point of the PointNetwork.
pub struct VolumeCloneSpawner {
    /// Which volume to clone.
    cloned_volume: VolumeType,
}

impl VolumeCloneSpawner {
    /// Creates a VolumeCloneSpawner with a reference [VolumeType] to be cloned.
    pub fn new(cloned_volume: VolumeType) -> Self {
        Self { cloned_volume }
    }
}

impl VolumeSpawner for VolumeCloneSpawner {
    fn volume_type_at(&self, _point: &PhysPoint, _point_idx: usize) -> VolumeType {
        self.cloned_volume
    }
}

impl<F: Fn(&PhysPoint, usize) -> VolumeType> VolumeSpawner for F {
    fn volume_type_at(&self, point: &PhysPoint, point_idx: usize) -> VolumeType {
        self(point, point_idx)
    }
}

impl VolumeCollection {
    /// Creates a [VolumeCollection] from a [PointNetwork] arbitrarily.
    ///
    /// Which volume to be spawned at which point is determined by a VolumeSpawner.
    ///
    /// Uses a predicate to determine which points should have volumes attached
    /// to them.
    pub fn at_points_when<Pred>(
        point_net: &PointNetwork,
        volume_spawner: impl VolumeSpawner,
        predicate: Pred,
    ) -> Self
    where
        Pred: Fn(&PhysPoint, usize) -> bool,
    {
        let volumes = point_net
            .points
            .iter()
            .enumerate()
            .filter_map(|(idx, point)| {
                if predicate(point, idx) {
                    Some(PhysicsVolume {
                        point_idx: idx,
                        volume_type: volume_spawner.volume_type_at(point, idx),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Self { volumes }
    }

    /// Creates a [VolumeCollection] from a [PointNetwork] at every point.
    ///
    /// Which volume to be spawned at which point is determined by a VolumeSpawner.
    pub fn at_every_point(point_net: &PointNetwork, volume_spawner: impl VolumeSpawner) -> Self {
        Self::at_points_when(point_net, volume_spawner, |_, _| true)
    }

    /// Creates a [VolumeCollection] from a [PointNetwork] at points that match
    /// certain indices.
    ///
    /// Which volume to be spawned at which point is determined by a VolumeSpawner.
    pub fn at_some_points_indexed(
        point_net: &PointNetwork,
        volume_spawner: impl VolumeSpawner,
        indices: &[usize],
    ) -> Self {
        Self::at_points_when(point_net, volume_spawner, |_, idx| indices.contains(&idx))
    }

    /// Get the full axis-aligned bounding box of every volume in this
    /// VolumeCollection.
    ///
    /// ## Panics
    ///
    /// If there are no volumes, this will panic, due to a unwrap on the
    /// return value of reduce.
    pub fn aabb(&self, point_net: &PointNetwork) -> AABB {
        self.volumes
            .iter()
            .map(|vol| {
                vol.volume_type
                    .aabb()
                    .translate(point_net.points[vol.point_idx].pos)
            })
            .reduce(|a, b| a.union(b))
            .unwrap()
    }
}
