use collision::CompositeCollider;
// NOTE: separate collider system from physics sistem
// (use rigid/softbody system to compose geom primitives, use physics sys to
//  give them inertia individually, use collision sys to find and handle
//  collisions)

use soa_derive::StructOfArray;
use ultraviolet::Vec3;

pub mod collision;
pub mod spring;

pub struct PhysicsContext; // WIP

#[derive(StructOfArray)]
struct PhysicsObjectList {
    pub pos: Vec3,
    pub vel: Vec3,
    pub collision: CompositeCollider,
    pub mass: f32,
}

pub struct PhysicsSimulation {
    objects: PhysicsObjectList,
} // WIP
