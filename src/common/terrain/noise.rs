//! # Simplex noise.
//!
//! This two-dimensional noise function is used by the terrain generator.

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

use std::fmt::Debug;

use rand::Rng;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct NoiseLatticePoint {
    pub inf_vec_x: f32,
    pub inf_vec_y: f32,
}

impl NoiseLatticePoint {
    pub fn new(inf_vec_x: f32, inf_vec_y: f32) -> Self {
        *Self {
            inf_vec_x,
            inf_vec_y,
        }
        .renormalize()
    }

    fn renormalize(&mut self) -> &mut Self {
        let mag = (self.inf_vec_x.powi(2) + self.inf_vec_y.powi(2)).sqrt();
        self.inf_vec_x /= mag;
        self.inf_vec_y /= mag;
        self
    }

    fn randomize<R: Rng + ?Sized>(&mut self, rng: &mut R) {
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

#[derive(Clone, PartialEq, Debug)]
pub struct LatticeQuadCorners {
    pub nw: NoiseLatticePoint,
    pub ne: NoiseLatticePoint,
    pub sw: NoiseLatticePoint,
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
    pub fn new(
        nw: NoiseLatticePoint,
        ne: NoiseLatticePoint,
        sw: NoiseLatticePoint,
        se: NoiseLatticePoint,
    ) -> Self {
        Self { nw, ne, sw, se }
    }

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

pub struct NoiseLattice {
    points: Vec<NoiseLatticePoint>,
    width: usize,
}

impl NoiseLattice {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            points: vec![NoiseLatticePoint::default(); width * height],
            width,
        }
    }

    pub fn height(&self) -> usize {
        self.points.len() / self.width
    }

    pub fn corners_at_quad(&self, qx: usize, qy: usize) -> LatticeQuadCorners {
        debug_assert!(qx < self.width - 1);
        debug_assert!(qy < self.height() - 1);

        let c_nw = qx + qy * self.width;
        let c_sw = qx + (qy + 1) * self.width;

        LatticeQuadCorners {
            nw: self.points[c_nw],
            ne: self.points[c_nw + 1],
            sw: self.points[c_sw],
            se: self.points[c_sw + 1],
        }
    }

    pub fn randomize(&mut self, rng: &mut impl rand::Rng) {
        self.points
            .iter_mut()
            .for_each(|point| point.randomize(rng));
    }

    pub fn get_influence_at(&self, pos_x: f32, pos_y: f32) -> f32 {
        let quad_x = pos_x.floor();
        let quad_y = pos_y.floor();
        let inner_x = pos_x.fract();
        let inner_y = pos_y.fract();

        self.corners_at_quad(quad_x as usize, quad_y as usize)
            .influence_at(inner_x, inner_y)
    }
}

// [TODO] fractal noise

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
}
