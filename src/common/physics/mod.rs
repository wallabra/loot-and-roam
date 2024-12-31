use collision::CompositeCollider;
use soa_derive::StructOfArray;
use ultraviolet::Vec3;

pub mod collision;

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
