//! # Perlin & Fractal Perlin noise
//!
//! This two-dimensional noise function is used by the terrain generator.
//!
//! We have a simple Perlin-type noise generator, [NoiseLattice], which
//! operates on a grid of lattice points each of which has a single
//! unit-length 'gradient vector'. Stacking noise lattices with varying
//! resolutions and strengths, we can get fractal noise, here implemented
//! as [FractalNoise] which enforces power-of-two 'octaves'.

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

use std::{fmt::Debug, num::NonZeroU16};

use rand::Rng;

#[derive(Default, Clone, Copy, PartialEq)]
/// A point on the grid of lattice points.
pub struct NoiseLatticePoint {
    inf_vec_x: f32,
    inf_vec_y: f32,
}

impl NoiseLatticePoint {
    /// Creates a new lattice grid point, with the x and y coordinate sof its
    /// gradient vector.
    pub fn new(inf_vec_x: f32, inf_vec_y: f32) -> Self {
        *Self {
            inf_vec_x,
            inf_vec_y,
        }
        .renormalize()
    }

    /// Returns the x and y coordinates of the 'gradient vector' at this
    /// lattice grid point.
    pub fn get_gradient_vector(&self) -> (f32, f32) {
        return (self.inf_vec_x, self.inf_vec_y);
    }

    fn renormalize(&mut self) -> &mut Self {
        let mag = (self.inf_vec_x.powi(2) + self.inf_vec_y.powi(2)).sqrt();
        self.inf_vec_x /= mag;
        self.inf_vec_y /= mag;
        self
    }

    /// Randomizes this lattice grid point using a [Rng].
    pub fn randomize<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        // Use polar randomization to ensure unit length
        let angle: f32 = rng.random_range(0.0..std::f32::consts::TAU);
        self.inf_vec_x = angle.cos();
        self.inf_vec_y = angle.sin();
    }

    fn influence_on(&self, off_x: f32, off_y: f32) -> f32 {
        self.inf_vec_x * off_x + self.inf_vec_y * off_y
    }
}

impl Debug for NoiseLatticePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?},{:?})", self.inf_vec_x, self.inf_vec_y)?;
        Ok(())
    }
}

/// A 'quad', or tile, of [NoiseLatticePoint] corners.
///
/// Note that +X is east and +Y is south.
#[derive(Clone, PartialEq, Debug)]
pub struct LatticeQuadCorners {
    /// The northwest point, or x:0 y:0.
    pub nw: NoiseLatticePoint,

    /// The northeast point, or x:1 y:0.
    pub ne: NoiseLatticePoint,

    /// The southwest point, or x:0 y:1.
    pub sw: NoiseLatticePoint,

    /// The southeast point, or x:1 y:1.
    pub se: NoiseLatticePoint,
}

fn lerp(from: f32, to: f32, alpha: f32) -> f32 {
    from + alpha * (to - from)
}

fn smootherstep(from: f32, to: f32, alpha: f32) -> f32 {
    let alpha = alpha * alpha * alpha * (alpha * (6.0 * alpha - 15.0) + 10.0);
    lerp(from, to, alpha)
}

impl LatticeQuadCorners {
    /// Creates a square lattice tile, or 'quad', from four [NoiseLatticePoint]
    /// definitions, one for each corner.
    pub fn new(
        nw: NoiseLatticePoint,
        ne: NoiseLatticePoint,
        sw: NoiseLatticePoint,
        se: NoiseLatticePoint,
    ) -> Self {
        Self { nw, ne, sw, se }
    }

    /// Calculates the value for each tile corner at the given input
    /// coordinates, then performs a smoothed bilinear interpolation
    /// between them.
    pub fn influence_at(&self, off_x: f32, off_y: f32) -> f32 {
        debug_assert!(off_x >= 0.0);
        debug_assert!(off_y >= 0.0);
        debug_assert!(off_x < 1.0);
        debug_assert!(off_y < 1.0);

        let inf_nw = self.nw.influence_on(off_x, off_y);
        let inf_ne = self.ne.influence_on(off_x - 1.0, off_y);
        let inf_sw = self.sw.influence_on(off_x, off_y - 1.0);
        let inf_se = self.se.influence_on(off_x - 1.0, off_y - 1.0);

        let inf_n = smootherstep(inf_nw, inf_ne, off_x);
        let inf_s = smootherstep(inf_sw, inf_se, off_x);

        smootherstep(inf_n, inf_s, off_y)
    }
}

/// A Perlin noise grid lattice.
///
/// Perlin noise is generated in a square grid.
///
/// Noise values are generated between the points on the grid, by finding
/// the dot product between the offset vector (from the grid point and the
/// input point) and the gradient vector of the grid point, then using a
/// bilinear interpolation (so closer grid points have more influence).
/// Note that the dot product (and thus the output) is zero when exactly on
/// grid points.
pub struct NoiseLattice {
    points: Vec<NoiseLatticePoint>,
    width: usize,
}

impl NoiseLattice {
    /// Creates a square lattice of width x height zeroed points.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            points: vec![NoiseLatticePoint::default(); width * height],
            width,
        }
    }

    /// Gets the width of the lattice in points.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Gets the height of the lattice in points.
    pub fn get_height(&self) -> usize {
        self.points.len() / self.width
    }

    /// Gets the [LatticeQuadCorners], or corners of the square tile, at the
    /// 'quad coordinates' qx and qy.
    pub fn corners_at_quad(&self, qx: usize, qy: usize) -> LatticeQuadCorners {
        debug_assert!(qx < self.width - 1);
        debug_assert!(qy < self.get_height() - 1);

        let c_nw = qx + qy * self.width;
        let c_sw = qx + (qy + 1) * self.width;

        LatticeQuadCorners {
            nw: self.points[c_nw],
            ne: self.points[c_nw + 1],
            sw: self.points[c_sw],
            se: self.points[c_sw + 1],
        }
    }

    /// Randomizes the gradient vectors of the lattice points, using the
    /// passed [Rng].
    pub fn randomize(&mut self, rng: &mut impl rand::Rng) {
        self.points
            .iter_mut()
            .for_each(|point| point.randomize(rng));
    }

    /// Gets the noise value at the given X and Y input coordinates.
    ///
    /// Note that those coordinates are always normalized such that tile
    /// corners are always on whole numbers. For instance, the top left quad
    /// starts at x=0.0,y=0.0 and ends at x=1.0,y=1.0, so all input coordinates
    /// within _those_ range will be interpolated between the corners of _that_
    /// quad.
    pub fn get_influence_at(&self, pos_x: f32, pos_y: f32) -> f32 {
        let quad_x = pos_x.floor();
        let quad_y = pos_y.floor();
        let inner_x = pos_x.fract();
        let inner_y = pos_y.fract();

        self.corners_at_quad(quad_x as usize, quad_y as usize)
            .influence_at(inner_x, inner_y)
    }
}

struct FractalNoiseOctave {
    lattice: NoiseLattice,
    octave: u16,
    resolution: f32,
}

impl FractalNoiseOctave {
    pub fn new(lattice: NoiseLattice, octave: u16) -> Self {
        Self {
            lattice,
            octave,
            resolution: 2.0_f32.powi(octave.into()),
        }
    }

    pub fn get_width(&self) -> f32 {
        (self.lattice.get_width() as f32) / self.resolution - 1.0
    }

    pub fn get_height(&self) -> f32 {
        (self.lattice.get_height() as f32) / self.resolution - 1.0
    }

    pub fn get_octave_scale(&self) -> f32 {
        self.resolution
    }

    pub fn get_octave(&self) -> u16 {
        self.octave
    }

    pub fn get_influence_at(&self, pos_x: f32, pos_y: f32) -> f32 {
        debug_assert!(pos_x < self.get_width());
        debug_assert!(pos_y < self.get_height());

        self.lattice
            .get_influence_at(pos_x * self.resolution, pos_y * self.resolution)
            / self.resolution
    }
}

/// Fractal Perlin noise.
///
/// This internally stacks multiple [NoiseLattices], each at a higher frequency
/// but smaller overall influence than the previous one, in order to make a
/// more "detailed" noise map.
///
/// This algorithm is known as 'fractal noise' because, if extended infinitely,
/// 'zooming in' an octave would look just as detailed, just like a fractal!
pub struct FractalNoise {
    width: f32,
    height: f32,
    octaves: Vec<FractalNoiseOctave>,
    max_octave: i32,
}

impl FractalNoise {
    /// Creates a fractal Perlin noise generator.
    ///
    /// The given width and height parameters are boundaries for the input x
    /// and y coordinates.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            octaves: vec![],
            max_octave: -1,
        }
    }

    fn update_max_octave(&mut self) {
        self.max_octave = self
            .octaves
            .iter()
            .map(|layer| layer.get_octave() as i32)
            .reduce(i32::max)
            .unwrap_or(if self.octaves.is_empty() { -1 } else { 0 });
    }

    /// Get the width of the input coordinate boundary; that is, the maximum
    /// x coordinate.
    pub fn get_width(&self) -> f32 {
        self.width
    }

    /// Get the height of the input coordinate boundary; that is, the maximum
    /// y coordinate.
    pub fn get_height(&self) -> f32 {
        self.height
    }

    /// Add a layer of Perlin noise at the given octave, using an initializer
    /// function.
    pub fn add_octave<T: FnMut(&mut NoiseLattice)>(
        &mut self,
        octave: u16,
        mut initializer: T,
    ) -> &mut Self {
        let span = 2.0_f32.powi(octave.into());
        let octave_width = (self.width * span + 1.0).floor() as usize;
        let octave_height = (self.height * span + 1.0).floor() as usize;

        let mut lattice = NoiseLattice::new(octave_width, octave_height);
        initializer(&mut lattice);

        self.octaves.push(FractalNoiseOctave::new(lattice, octave));
        self.update_max_octave();
        self
    }

    /// Add a layer of Perlin noise at the given octave, initializing it
    /// randomly using the passed [Rng].
    pub fn add_random_octave(&mut self, octave: u16, rng: &mut impl Rng) -> &mut Self {
        self.add_octave(octave, move |layer| layer.randomize(rng));
        self
    }

    /// Helper function to add many octaves at once, all with the same
    /// initializer function.
    ///
    /// Every octave will start off after the last/finest octave; if there are
    /// no octaves present, they will start at octave zero.
    pub fn add_many_octaves<T: FnMut(&mut NoiseLattice)>(
        &mut self,
        num_octaves: NonZeroU16,
        mut initializer: T,
    ) -> &mut Self {
        for octave in
            self.max_octave as u16 + 1..=self.max_octave as u16 + u16::from(num_octaves) + 1
        {
            self.add_octave(octave, &mut initializer);
        }
        self
    }

    /// Helper function to add many octaves, randomly initializing all of them
    /// with the same passed [Rng].
    ///
    /// Every octave will start off after the last/finest octave; if there are
    /// no octaves present, they will start at octave zero.
    pub fn add_many_random_octaves(
        &mut self,
        num_octaves: NonZeroU16,
        rng: &mut impl Rng,
    ) -> &mut Self {
        self.add_many_octaves(num_octaves, move |layer| layer.randomize(rng))
    }

    /// Get the noise value at the input coordinates pos_x and pos_y.
    pub fn get_influence_at(&self, pos_x: f32, pos_y: f32) -> f32 {
        self.octaves
            .iter()
            .map(|oct| oct.get_influence_at(pos_x, pos_y))
            .sum()
    }
}

pub mod tests {
    #[test]
    fn quad_lookup() {
        use super::NoiseLattice;

        let lattice = NoiseLattice::new(4, 3);
        let quad_1 = lattice.corners_at_quad(1, 0);
        let quad_2 = lattice.corners_at_quad(1, 1);
        let quad_3 = lattice.corners_at_quad(2, 0);

        assert_eq!(quad_1.sw, quad_2.nw);
        assert_eq!(quad_1.se, quad_2.ne);
        assert_eq!(quad_1.ne, quad_3.nw);
        assert_eq!(quad_1.se, quad_3.sw);
        assert_eq!(
            quad_1.influence_at(0.5, 0.99999),
            quad_2.influence_at(0.5, 0.0)
        );
    }

    #[test]
    fn fractal_noise() {
        use super::FractalNoise;

        let mut fractal = FractalNoise::new(1.0, 1.0);
        let mut rng = rand::rng();

        fractal.add_random_octave(0, &mut rng);
        fractal.add_random_octave(1, &mut rng);
        fractal.add_random_octave(2, &mut rng);

        let inf_1 = fractal.get_influence_at(0.5, 0.5);

        fractal.add_random_octave(12, &mut rng);

        let inf_2 = fractal.get_influence_at(0.5, 0.5);

        assert!((inf_1 - inf_2).abs() <= 2.0_f32.powi(-12));
    }
}
