//! # Terrain buffer.
//!
//! A terrain heightmap can be meshed.

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

use crate::common::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

/// A terrain buffer.
///
/// Stores a heightmap with resolution. Can be made from a [TerrainGenerator]
/// using its [generate] constructor, and then a [Mesh] can be made from it
/// using [to_mesh].
pub struct TerrainBuffer {
    /// The spacing, in world space units, between vertices.
    resolution: f32,

    /// The width of the 2D heightmap sample array.
    width: usize,

    /// The height of the 2D heightmap sample array.
    height: usize,

    /// The data of the 2D heightmap sample array.
    values: Vec<f32>,
}

impl TerrainBuffer {
    pub fn get_vert_width(&self) -> usize {
        self.width
    }

    pub fn get_real_width(&self) -> f32 {
        self.width as f32 * self.resolution
    }

    pub fn get_vert_height(&self) -> usize {
        self.height
    }

    pub fn get_real_height(&self) -> f32 {
        self.height as f32 * self.resolution
    }

    pub fn get_num_tris(&self) -> usize {
        (self.get_vert_width() - 1) * (self.get_vert_height() - 1) * 2
    }

    pub fn generate<TMA, DC>(
        generator: TerrainGenerator<TMA, DC>,
        resolution: f32,
        scale: f32,
        vert_scale: f32,
    ) -> Self
    where
        TMA: TerrainModulatorAlgorithm,
        DC: DistanceCollector,
    {
        let width = (generator.get_width() / resolution).floor() as usize;
        let height = (generator.get_height() / resolution).floor() as usize;

        debug_assert!(width > 1);
        debug_assert!(height > 1);

        let values = (0_usize..width * height)
            .map(|idx| {
                let x = idx % width;
                let y = idx / width;
                let x = x as f32 * resolution;
                let y = y as f32 * resolution;

                generator.get_height_at(Vec2::new(x, y)) * vert_scale
            })
            .collect::<Vec<_>>();

        Self {
            width,
            height,
            resolution: scale,
            values,
        }
    }

    pub fn get_value_at(&self, value_x: usize, value_y: usize) -> f32 {
        self.values[value_y * self.get_vert_width() + value_x]
    }

    pub fn to_mesh(&self) -> Mesh {
        debug_assert!(self.width > 1);
        debug_assert!(self.height > 1);

        let quad_width = self.get_vert_width() - 1;
        let center_x = self.get_real_width() / 2.0;
        let center_y = self.get_real_height() / 2.0;

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            (0..self.get_num_tris() * 3)
                .map(|vertex_idx| {
                    let tri_idx = vertex_idx / 3;
                    let vert_in_tri = vertex_idx % 3;
                    let quad_idx = tri_idx / 2;

                    // vertex quad, not perlin quad
                    let quad_x = quad_idx % quad_width;
                    let quad_y = quad_idx / quad_width;

                    use QuadCorner::*;

                    let which_corner = match tri_idx % 2 {
                        0 => {
                            // even triangles: NW, NE, SW
                            [NE, NW, SW][vert_in_tri]
                        }
                        1 => {
                            // odd triangles: NE, SW, SE
                            [NE, SW, SE][vert_in_tri]
                        }
                        _ => unreachable!(),
                    };

                    // horizontal
                    let value_x = quad_x + which_corner.x();
                    let value_y = quad_y + which_corner.y();
                    let vert_x = value_x as f32 * self.resolution - center_x;
                    let vert_z = value_y as f32 * self.resolution - center_y;
                    // vertical
                    let vert_y = self.get_value_at(value_x, value_y);

                    [vert_x, vert_y, vert_z]
                })
                .collect::<Vec<_>>(),
        )
        .with_inserted_indices(Indices::U32(
            (0_u32..(self.get_num_tris() * 3) as u32).collect::<Vec<_>>(),
        ))
        .with_computed_normals()
    }

    /// Create an entity bundle from this Terrain.
    pub fn as_bundle(self, meshes: &mut ResMut<Assets<Mesh>>) -> impl Bundle {
        let mesh = self.to_mesh();
        return (Mesh3d(meshes.add(mesh)), TerrainMarker::new(self));
    }
}

/// Marks an entity as a terrain.
///
/// It msut hold a [TerrainBuffer].
///
/// Only a single Terrain entity will be loaded by the terrain renderer.
#[derive(Component)]
pub struct TerrainMarker {
    /// The buffer of this terrain.
    pub buffer: TerrainBuffer,
}

impl TerrainMarker {
    /// Construct a new TerrainMarker and initialize it with a [TerrainBuffer].
    pub fn new(buffer: TerrainBuffer) -> Self {
        Self { buffer }
    }
}
