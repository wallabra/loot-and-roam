//! # Terrain collision system

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

use crate::common::{
    physics::collision::CollisionDetectionEvent,
    prelude::{CollisionInfo, PhysicsVolume, PointNetwork, VolumeCollection, AABB},
};

use super::buffer::{TerrainBuffer, TerrainMarker};

/// AABB of a given terrain, in its local coordinate space.
fn terrain_aabb(buffer: &TerrainBuffer) -> AABB {
    AABB::new(
        -buffer.get_real_width() / 2.0..buffer.get_real_width() / 2.0,
        buffer.get_vertical_height_range(),
        -buffer.get_real_height() / 2.0..buffer.get_real_height() / 2.0,
    )
}

/// Event emitted when a volumed object collides with a terrain entity.
#[derive(Event)]
pub struct TerrainVolumeCollisionDetectionEvent {
    /// The volumed entity on the collision check.
    ///
    /// Parameters like [info] are from the perspective of this entity.
    pub entity_ref: Entity,

    /// The terrain entity on the collision check.
    pub entity_terrain: Entity,

    /// The volume on the volumed entity for which collision was detected.
    ///
    /// This is a clone, so any changes on it will *not* be reflected on the
    /// original entity. That has to be done externally, e.g. by accessing
    /// [entity_ref] directly on the system's Commands.
    pub volume: PhysicsVolume,

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

impl CollisionDetectionEvent for TerrainVolumeCollisionDetectionEvent {
    fn perspective_entity(&self) -> Entity {
        self.entity_ref
    }

    fn other_entity(&self) -> Entity {
        self.entity_terrain
    }

    fn info(&self) -> &CollisionInfo {
        &self.info
    }

    fn depth(&self) -> f32 {
        self.depth
    }
}

/// Terrain-object collision via physics volumes.
fn terrain_volume_collision_system(
    mut ev_collision: EventWriter<TerrainVolumeCollisionDetectionEvent>,
    mut query: Query<(Entity, &mut PointNetwork, &VolumeCollection), Without<TerrainMarker>>,
    terrain_query: Query<(Entity, &TerrainMarker, &Transform)>,
) {
    for (e1, mut points1, volumes1) in query.iter_mut() {
        // [NOTE] For more info on the below comment on loop label, see note below
        // near its continue.

        // 'detect_loop:
        for (e2, terramark, terratransf) in terrain_query.iter() {
            let terrabuf = &terramark.buffer;
            let terrabox = terrain_aabb(&terrabuf);

            if !volumes1.aabb(&points1).check(&terrabox) {
                continue;
            }

            for vol in &volumes1.volumes {
                let pos = points1.points[vol.point_idx].pos;

                // Point pssition mapped to the terrain's local space.
                let pos_mapped = terratransf.compute_matrix().inverse().transform_point3(pos);

                // AABB check
                if !terrabox.check_point(pos) {
                    continue;
                }

                // Terrain height check
                let terra_height = terrabuf.get_height_at(pos_mapped.x, pos_mapped.z);

                if pos_mapped.y > terra_height {
                    continue;
                }

                // Depth is how far into the ground the point is.
                let depth = terra_height - pos_mapped.y;

                // Normal is based on the gradient, which is brute forced by
                // interpolating terrain values at offset positions in a
                // weighted manner.
                // [TODO] Analytical Perlin noise differentiation
                let normal = terrabuf.get_normal_at(pos_mapped.x, pos_mapped.z);
                let normal_global = terratransf.transform_point(normal) - terratransf.translation;

                let collision = CollisionInfo {
                    pos: terratransf.transform_point(pos_mapped + Vec3::Z * (depth / 2.0)),
                    normal: normal_global,
                };

                points1.points[vol.point_idx].vel += normal_global * depth;

                ev_collision.write(TerrainVolumeCollisionDetectionEvent {
                    entity_ref: e1,
                    entity_terrain: e2,
                    info: collision,
                    depth,
                    volume: vol.clone(),
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

// [TODO] [after:terrain] Add volume-terrain collision

pub struct TerrainCollisionPlugin;

impl Plugin for TerrainCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (terrain_volume_collision_system,));
        app.add_event::<TerrainVolumeCollisionDetectionEvent>();
    }
}
