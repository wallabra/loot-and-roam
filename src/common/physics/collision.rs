//! Collision systems.

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

use super::base::PointNetwork;

// [TODO] Volume collisions
//        Waiting on:  volume implementation

// Floor plane collisions
#[derive(Default, Component)]
pub struct FloorPlaneCollision {
    pub intercept_y: f32,
    pub restitution: f32,
    pub friction: f32,
}

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

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, floor_plane_collision_system);
    }
}
