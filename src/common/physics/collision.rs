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
    volume::{CollisionInfo, PhysicsVolume, VolumeCollection, VolumeCollision, VolumeInfo},
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

/// Event emitted when two volumed objects collide.
#[derive(Event)]
pub struct VolumeVolumeCollisionDetectionEvent {
    /// The first entity on the collision check.
    ///
    /// Parameters like [info] are from the perspective of this entity.
    pub entity_ref: Entity,

    /// The second entity on the collision check.
    pub entity_other: Entity,

    /// The volume on the first entity for which collision was detected.
    ///
    /// This is a clone, so any changes on it will *not* be reflected on the
    /// original entity. That has to be done externally, e.g. by accessing
    /// [entity_ref] directly on the system's Commands.
    pub volume_1: PhysicsVolume,

    /// The volume on the second entity for which collision was detected.
    ///
    /// This is a clone, so any changes on it will *not* be reflected on the
    /// original entity. That has to be done externally, e.g. by accessing
    /// [entity_other] directly on the system's Commands.
    pub volume_2: PhysicsVolume,

    /// Collision info, such as relative position and collision normal.
    ///
    /// Note that this is relative to the first entity, [entity_ref].
    pub info: CollisionInfo,

    /// Collision depth.
    ///
    /// An average of the depth calculated from both volumes based on their
    /// SDFs.
    pub depth: f32,
}

/// Object-object collision via physics volumes.
fn volume_volume_collision_system(
    mut ev_collision: EventWriter<VolumeVolumeCollisionDetectionEvent>,
    mut query: Query<(Entity, &mut PointNetwork, &VolumeCollection)>,
) {
    // [TODO] Replace global all-pair combination iteration with a spatially accelerated data structure.
    let mut combinations = query.iter_combinations_mut();

    // [NOTE] For more info on the below comment on loop label, see note below
    // near its continue.

    // 'detect_loop:
    while let Some([(e1, mut points1, volumes1), (e2, mut points2, volumes2)]) =
        combinations.fetch_next()
    {
        if !volumes1.aabb(&points1).check(volumes2.aabb(&points2)) {
            continue;
        }

        for vol1 in &volumes1.volumes {
            let pos1 = points1.points[vol1.point_idx].pos;

            for vol2 in &volumes2.volumes {
                let pos2 = points2.points[vol2.point_idx].pos;
                let offs_1_to_2 = pos2 - pos1;

                let collision = vol1.volume_type.collision(&vol2.volume_type, offs_1_to_2);

                if let Some(collision) = collision {
                    // Depth is average of SDF-based depth on both entities
                    let depth = (-vol1.volume_type.sdf(collision.pos)
                        - vol2.volume_type.sdf(collision.pos - offs_1_to_2))
                        / 2.0;

                    //info!("Handling collision of depth {}", depth);

                    // points1.points[vol1.point_idx].pos -= collision.normal * depth;
                    points1.points[vol1.point_idx].vel -= collision.normal * depth;

                    // points2.points[vol2.point_idx].pos += collision.normal * depth;
                    points2.points[vol2.point_idx].vel += collision.normal * depth;

                    ev_collision.send(VolumeVolumeCollisionDetectionEvent {
                        entity_ref: e1,
                        entity_other: e2,
                        info: collision,
                        depth,
                        volume_1: vol1.clone(),
                        volume_2: vol2.clone(),
                    });

                    // [NOTE] Uncomment the following to handle only one
                    // volume-volume interaction at a time. Might help in terms
                    // of performance and reducing "redundant" collision
                    // events, but will likely lead to worse collision
                    // resolution overall.

                    // continue 'detect_loop;
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
        app.add_event::<VolumeVolumeCollisionDetectionEvent>();
    }
}
