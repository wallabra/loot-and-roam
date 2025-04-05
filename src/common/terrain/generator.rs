//! # Terrain generator
//!
//! Uses the fractal Perlin noise to generate island heightmaps.
//!
//! Heights far enough from a center point is pushed underwater.
//! This is accomplished through a heightmap transform function, which forces
//! height values to become smaller than 0 if far enough from any of the
//! island's "center points".

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

use bevy::math::Vec2;
use derive_builder::Builder;

use crate::common::math::{lerp, smootherstep};

use super::noise::FractalNoise;

/// The parameters used when modulating terrain height.
#[derive(Clone, Debug)]
pub struct ModulationParams {
    /// The distance around center points outside of which should be
    /// underwater.
    pub max_shore_distance: f32,

    /// The radius away from a center point which should be guaranteed to be
    /// above the water.
    pub min_shore_distance: f32,

    /// The minimum height of land (above water) terrain, between 0 and 1.
    pub shore_rim: f32,

    /// The maximum height of underwater terrain, between -1 and 0.
    pub seabed_rim: f32,
}

impl Default for ModulationParams {
    fn default() -> Self {
        Self {
            min_shore_distance: 100.0,
            max_shore_distance: 300.0,
            shore_rim: 0.1,
            seabed_rim: -0.4,
        }
    }
}

/// A terrain height modulation algorithm.
///
/// Not knowing about the actual center points, this algorithm is only given
/// the 'collected' distance.
pub trait TerrainModulatorAlgorithm: Clone {
    /// Modulate a height based on the collected distance alone.
    fn push_terrain(&self, params: &ModulationParams, distance: f32, curr_height: f32) -> f32;
}

/// The detaulr terrain modulator algorithm.
#[derive(Clone, Debug)]
pub struct DefaultTerrainModulatorAlgorithm;

impl TerrainModulatorAlgorithm for DefaultTerrainModulatorAlgorithm {
    fn push_terrain(&self, params: &ModulationParams, distance: f32, curr_height: f32) -> f32 {
        // Use a spline to push terrain up or down.
        let pushed_up = lerp(params.shore_rim, 1.0, (curr_height + 1.0) / 2.0);
        let pushed_down = lerp(0.0, params.seabed_rim, (curr_height + 1.0) / 2.0);

        if distance < params.min_shore_distance {
            pushed_up
        } else if distance > params.max_shore_distance
            || params.max_shore_distance == params.min_shore_distance
        {
            pushed_down
        } else {
            // This division is why we check if max and min shore distances are equal above. :)
            let alpha = (distance - params.min_shore_distance)
                / (params.max_shore_distance - params.min_shore_distance);

            // [TODO] Generalize spline interpolation algorithm choice
            smootherstep(pushed_up, pushed_down, alpha)
        }
    }
}

/// A center point distance collector.
///
/// Each center point has its own distance to the given coordinates. This
/// trait allows picking out a single algorithm to collect them into a single
/// distance.
pub trait DistanceCollector: Clone {
    /// Collects multiple distances into one.
    ///
    /// ## Safety
    ///
    /// May panic if distances is empty.
    fn collect_distances(&self, distances: Vec<f32>) -> f32;
}

/// A simple distance collector; simply pick the smallest!
#[derive(Clone, Debug)]
pub struct MinDistance;

impl DistanceCollector for MinDistance {
    fn collect_distances(&self, distances: Vec<f32>) -> f32 {
        distances.into_iter().reduce(f32::min).unwrap()
    }
}

/// A smoothmin distance collector. Smoothes out the edges.
#[derive(Clone, Debug)]
pub struct SmoothminDistance {
    /// The 'roughness' of this smoothmin collector.
    ///
    /// The higher this value is, the wider the smoothening of the seams.
    ///
    /// A value of 1 is functionally equivalent to [MinDistance].
    pub roughness: f32,
}

impl Default for SmoothminDistance {
    fn default() -> Self {
        Self { roughness: 20.0 }
    }
}

impl DistanceCollector for SmoothminDistance {
    fn collect_distances(&self, distances: Vec<f32>) -> f32 {
        0.0_f32.max(
            (0.000001_f32
                .max(
                    distances
                        .into_iter()
                        .map(|v| (v * self.roughness).exp())
                        .sum(),
                )
                .ln())
                / self.roughness,
        )
    }
}

/// A terrain modulator.
///
/// Allows using any algorithm for modulating the terrain, and any algorithm
/// for collecting the distances between various center points into a single
/// distance to use for modulation.
#[derive(Clone, Debug)]
pub struct TerrainModulator<TMA, DC>
where
    TMA: TerrainModulatorAlgorithm + Sized,
    DC: DistanceCollector + Sized,
{
    algorithm: TMA,
    distance_collector: DC,
}

/// Gets a 'reasonable defaults' terrain modulator.
pub fn default_modulator() -> TerrainModulator<DefaultTerrainModulatorAlgorithm, SmoothminDistance>
{
    TerrainModulator {
        algorithm: DefaultTerrainModulatorAlgorithm,
        distance_collector: SmoothminDistance::default(),
    }
}

#[derive(Debug, Clone)]
/// A center point, around which shore distances should be set for
/// terrain height modulation.
pub struct CenterPoint {
    /// The coordinates of the center point.
    pos: Vec2,

    /// The 'scale' of the center point.
    ///
    /// Scales the shore distances of the modulation parameters.
    scale: f32,
}

impl CenterPoint {
    /// Makes a new CenterPoint at the given position and with the given scale.
    pub fn new(pos: Vec2, scale: f32) -> Self {
        Self { pos, scale }
    }
}

impl<TMA, DC> TerrainModulator<TMA, DC>
where
    TMA: TerrainModulatorAlgorithm + Sized,
    DC: DistanceCollector + Sized,
{
    /// Modulate terrain using the passed parameters, center points, input
    /// coordinates, and the current height.
    pub fn push_terrain(
        &self,
        params: &ModulationParams,
        center_points: &Vec<CenterPoint>,
        at: Vec2,
        curr_height: f32,
    ) -> f32 {
        let distances = center_points
            .iter()
            .map(|point| (point.pos - at).length() * point.scale)
            .collect::<Vec<_>>();
        let distance = self.distance_collector.collect_distances(distances);

        self.algorithm.push_terrain(params, distance, curr_height)
    }
}

/// The terrain generator.
///
/// Uses fractal Perlin noise to generate terrain values, and then uses a
/// list of center points to 'modulate' terrain - that is, making sure it is
/// underwater if it is too far from the center points.
///
/// This terrain generator outputs values between 0.0 and 1.0. Further
/// transformation may be desired depending on the desired vertical scale.
#[derive(Builder, Clone)]
pub struct TerrainGenerator<TMA, DC>
where
    TMA: TerrainModulatorAlgorithm + Sized,
    DC: DistanceCollector + Sized,
{
    /// The fractal Perlin noise genreator to use.
    noise: FractalNoise,

    /// The terrain modulator to use.
    modulator: TerrainModulator<TMA, DC>,

    /// The 'center points' around which to modulate terrain.
    center_points: Vec<CenterPoint>,

    /// The terrain modulation parameters.
    #[builder(default)]
    modulation_params: ModulationParams,

    /// The size of each noise 'tile' (at octave 0).
    // [NOTE] Change the below default value to change the size of terrain noise tiles!
    #[builder(default = 80.0)]
    resolution: f32,
}

impl<TMA, DC> TerrainGenerator<TMA, DC>
where
    TMA: TerrainModulatorAlgorithm,
    DC: DistanceCollector,
{
    /// Get the height of terrain generated at these coordinates.
    pub fn get_height_at(&self, at: Vec2) -> f32 {
        let height = self
            .noise
            .get_influence_at(at.x / self.resolution, at.y / self.resolution);

        let height =
            self.modulator
                .push_terrain(&self.modulation_params, &self.center_points, at, height);

        height
    }

    /// Get the bounding width of this terrain generator.
    pub fn get_width(&self) -> f32 {
        self.noise.get_width() * self.resolution
    }

    /// Get the bounding height of this terrain generator.
    pub fn get_height(&self) -> f32 {
        self.noise.get_height() * self.resolution
    }
}

pub type DefaultTerrainGenerator =
    TerrainGenerator<DefaultTerrainModulatorAlgorithm, SmoothminDistance>;

pub type DefaultTerrainGeneratorBuilder =
    TerrainGeneratorBuilder<DefaultTerrainModulatorAlgorithm, SmoothminDistance>;
