//! # Collision systems
//!
//! A few different kinds of collision are implemented here: primarily
//! object-object and object-terrain collision, as well as a few very basic
//! types (such as floor plane collision).

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

use super::{
    base::PointNetwork,
    volume::{VolumeCollection, VolumeCollision, VolumeInfo},
};

// [TODO] Volume collisions
//        Waiting on:  volume implementation

/// Floor plane collision component.
///
/// Use this on an entity for which you want every physics point to stay above
/// a certain Y value.
#[derive(Default, Component)]
pub struct FloorPlaneCollision {
    /// The Y value below which every physics point should be forced above.
    pub intercept_y: f32,

    /// How much of a physics point's downward velocity should be reflected
    /// upward when it is pushed up from under the intercept.
    pub restitution: f32,

    /// How much of a physics point's horizontal velocity should dissipate, in
    ///  Newtons per second, on a given frame that this point is found under
    /// the Y intercept plane.
    pub friction: f32,
}

/// Floor plane collision for physics points.
///
/// It guarantees that every physics point is above a certain Y intercept
/// value - by default 0.0.
fn floor_plane_collision_system(mut query: Query<(&mut PointNetwork, &FloorPlaneCollision)>) {
    for (mut points, collision) in query.iter_mut() {
        for point in &mut points.points {
            if point.pos.y < collision.intercept_y {
                point.pos.y = collision.intercept_y;
                point.vel.y *= -collision.restitution;

                let mut shift = point.vel * -collision.friction / point.mass;

                if shift.length_squared() > point.vel.length_squared() {
                    point.vel.x = 0.0;
                    point.vel.z = 0.0;
                } else {
                    shift.y = 0.0;
                    point.vel.x += shift.x;
                    point.vel.z += shift.z;
                }
            }
        }
    }
}

/// Object-object collision via physics volumes.
fn volume_volume_collision_system(mut query: Query<(&mut PointNetwork, &VolumeCollection)>) {
    // [TODO] Replace global all-pair combination iteration with a spatially accelerated data structure.
    let mut combinations = query.iter_combinations_mut();

    while let Some([(mut points1, volumes1), (mut points2, volumes2)]) = combinations.fetch_next() {
        for vol1 in &volumes1.volumes {
            let pos1 = points1.points[vol1.point_idx].pos;

            for vol2 in &volumes2.volumes {
                let pos2 = points2.points[vol2.point_idx].pos;
                let offs_1_to_2 = pos2 - pos1;

                let collision = vol1.volume_type.collision(&vol2.volume_type, offs_1_to_2);

                if let Some(collision) = collision {
                    let depth = -vol1.volume_type.sdf(collision.pos);

                    points1.points[vol1.point_idx].pos -= collision.normal * depth;
                    points1.points[vol1.point_idx].vel -= collision.normal * depth;

                    points2.points[vol2.point_idx].pos += collision.normal * depth;
                    points2.points[vol2.point_idx].vel += collision.normal * depth;
                }
            }
        }
    }
}

// [TODO] [after:terrain] Add volume-terrain collision

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (floor_plane_collision_system, volume_volume_collision_system),
        );
    }
}
