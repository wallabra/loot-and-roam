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

use crate::common::math::smootherstep;

use super::noise::FractalNoise;

/// Some of the parameters used when modulating terrain height.
///
/// Omits the inteprolator, which can be added in later.
#[derive(Clone, Debug, Builder)]
pub struct BaseModulationParams {
    /// The distance around center points outside of which should be
    /// underwater.
    pub max_shore_distance: f32,

    /// The radius away from a center point which should be guaranteed to be
    /// above the water.
    pub min_shore_distance: f32,

    /// The amount by which to conform terrain into the island forms.
    ///
    /// Smaller amounts let in more Perlin noise, higher amounts conform it
    /// more; too high and you get blobs!
    pub islandification: f32,
}

impl BaseModulationParams {
    /// Completes into a [ModulationParams] by adding the interpolator.
    pub fn with_interpolator<'fn_interp>(
        self,
        interpolator: &'fn_interp fn(f32, f32, f32) -> f32,
    ) -> ModulationParams<'fn_interp> {
        ModulationParams {
            max_shore_distance: self.max_shore_distance,
            min_shore_distance: self.min_shore_distance,
            islandification: self.islandification,
            interpolator,
        }
    }
}

/// The parameters used when modulating terrain height.
#[derive(Clone, Debug, Builder)]
pub struct ModulationParams<'fn_interp> {
    /// The distance around center points outside of which should be
    /// underwater.
    pub max_shore_distance: f32,

    /// The radius away from a center point which should be guaranteed to be
    /// above the water.
    pub min_shore_distance: f32,

    /// The amount by which to conform terrain into the island forms.
    ///
    /// Smaller amounts let in more Perlin noise, higher amounts conform it
    /// more; too high and you get blobs!
    pub islandification: f32,

    /// Function to use to interpolate between the
    /// Perlin height and the 'islandified' height.
    #[builder(default=&(smootherstep as fn(f32, f32, f32) -> f32))]
    pub interpolator: &'fn_interp fn(f32, f32, f32) -> f32,
}

impl<'a> Default for ModulationParams<'a> {
    fn default() -> Self {
        Self {
            min_shore_distance: 30.0,
            max_shore_distance: 80.0,
            islandification: 0.4,
            interpolator: &(smootherstep as fn(f32, f32, f32) -> f32),
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
        let scaled_distance = if distance < params.min_shore_distance {
            0.0
        } else {
            // let the result be > 1
            (distance - params.min_shore_distance)
                / (params.max_shore_distance - params.min_shore_distance)
        };

        let islandified_height = 1.0 - scaled_distance;

        (params.interpolator)(curr_height, islandified_height, params.islandification).max(-1.0)
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
    /// The higher this value is, the sharper the smoothening of the seams.
    ///
    /// A value of infinity would be functionally equivalent to [MinDistance],
    /// if computers were like magical unicorns.
    pub roughness: f32,
}

impl Default for SmoothminDistance {
    fn default() -> Self {
        Self { roughness: 1.5 }
    }
}

impl DistanceCollector for SmoothminDistance {
    fn collect_distances(&self, distances: Vec<f32>) -> f32 {
        0.0_f32.max(
            (-distances
                .into_iter()
                .map(|v| (-self.roughness as f64 * v as f64).exp())
                .sum::<f64>()
                .ln()
                / self.roughness as f64) as f32,
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
            .map(|point| (point.pos - at).length() / point.scale)
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
pub struct TerrainGenerator<'fn_interp, TMA, DC>
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
    modulation_params: ModulationParams<'fn_interp>,

    /// The size of each noise 'tile' (at octave 0).
    // [NOTE] Change the below default value to change the size of terrain noise tiles!
    #[builder(default = 200.0)]
    resolution: f32,
}

impl<'fn_interp, TMA, DC> TerrainGenerator<'fn_interp, TMA, DC>
where
    TMA: TerrainModulatorAlgorithm,
    DC: DistanceCollector,
{
    /// Get the height of terrain generated at these coordinates.
    pub fn get_height_at(&self, at: Vec2) -> f32 {
        let height = self
            .noise
            .get_influence_at(at.x / self.resolution, at.y / self.resolution);

        

        self.modulator
                .push_terrain(&self.modulation_params, &self.center_points, at, height)
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
    TerrainGenerator<'static, DefaultTerrainModulatorAlgorithm, SmoothminDistance>;

pub type DefaultTerrainGeneratorBuilder =
    TerrainGeneratorBuilder<'static, DefaultTerrainModulatorAlgorithm, SmoothminDistance>;
