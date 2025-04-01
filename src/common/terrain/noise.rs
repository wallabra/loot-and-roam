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

use rand::Rng;

#[derive(Default, Clone, Copy)]
pub struct NoiseLatticePoint {
    inf_vec_x: i8,
    inf_vec_y: i8,
}

impl NoiseLatticePoint {
    pub fn new(inf_vec_x: i8, inf_vec_y: i8) -> Self {
        Self {
            inf_vec_x,
            inf_vec_y,
        }
    }

    fn randomize<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        // Use polar randomization to ensure unit length
        let angle: f32 = rng.random_range(0.0..std::f32::consts::TAU);
        self.inf_vec_x = (angle.cos() * 128.0) as i8;
        self.inf_vec_y = (angle.sin() * 128.0) as i8;
    }

    fn influence_on_i8(&self, off_x: i8, off_y: i8) -> i8 {
        ((self.inf_vec_x as i16) * (off_x as i16 + ((off_x < 64) as i16)) / 128
            + (self.inf_vec_y as i16 + ((off_y < 64) as i16)) * (off_y as i16) / 128) as i8
    }

    fn influence_on_f32(&self, off_x: f32, off_y: f32) -> f32 {
        (self.inf_vec_x as f32 / 128.0) * off_x + (self.inf_vec_y as f32 / 128.0) * off_y
    }
}

pub struct LatticeQuadCorners {
    nw: NoiseLatticePoint,
    ne: NoiseLatticePoint,
    sw: NoiseLatticePoint,
    se: NoiseLatticePoint,
}

fn smootherstep_i8(from: i8, to: i8, alpha: u8) -> i8 {
    let alpha = (alpha as f32) / 256.0;
    let alpha = alpha * alpha * alpha * (alpha * (6.0 * alpha - 15.0) + 10.0);

    (from as i32 + (alpha * (to as f32 - from as f32)) as i32) as i8
}

fn smootherstep_f32(from: f32, to: f32, alpha: f32) -> f32 {
    let alpha = alpha * alpha * alpha * (alpha * (6.0 * alpha - 15.0) + 10.0);
    from + alpha * (to - from)
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

    pub fn influence_at_i8(&self, off_x: i8, off_y: i8) -> i8 {
        debug_assert!(off_x >= 0);
        debug_assert!(off_y >= 0);

        let inf_nw = self.nw.influence_on_i8(off_x, off_y);
        let inf_ne = self.ne.influence_on_i8(off_x - 127, off_y);
        let inf_sw = self.sw.influence_on_i8(off_x, off_y - 127);
        let inf_se = self.se.influence_on_i8(off_x - 127, off_y - 127);

        let inf_n = smootherstep_i8(inf_nw, inf_ne, off_x as u8 * 2);
        let inf_s = smootherstep_i8(inf_sw, inf_se, off_x as u8 * 2);

        smootherstep_i8(inf_n, inf_s, off_y as u8 * 2)
    }

    pub fn influence_at_f32(&self, off_x: f32, off_y: f32) -> f32 {
        debug_assert!(off_x >= 0.0);
        debug_assert!(off_y >= 0.0);
        debug_assert!(off_x < 1.0);
        debug_assert!(off_y < 1.0);

        let inf_nw = self.nw.influence_on_f32(off_x, off_y);
        let inf_ne = self.ne.influence_on_f32(off_x - 1.0, off_y);
        let inf_sw = self.sw.influence_on_f32(off_x, off_y - 1.0);
        let inf_se = self.se.influence_on_f32(off_x - 1.0, off_y - 1.0);

        let inf_n = smootherstep_f32(inf_nw, inf_ne, off_x);
        let inf_s = smootherstep_f32(inf_sw, inf_se, off_x);

        smootherstep_f32(inf_n, inf_s, off_y)
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

    fn corners_at_quad(&self, qx: usize, qy: usize) -> LatticeQuadCorners {
        assert!(qx < self.width - 1);
        assert!(qy < self.height() - 1);

        let c_nw = qy * self.width + qx;
        let c_sw = (qy * self.width + 1) + qx;

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

    /// The last 7 bits of each value are the fractional part of each quadrant.
    pub fn get_influence_at_u16(&self, pos_x: u16, pos_y: u16) -> i8 {
        let quad_x = (pos_x >> 8) as usize;
        let quad_y = (pos_y >> 8) as usize;
        let inner_x = (pos_x & 0x7F) as i8;
        let inner_y = (pos_y & 0x7F) as i8;

        self.corners_at_quad(quad_x, quad_y)
            .influence_at_i8(inner_x, inner_y)
    }

    pub fn get_influence_at_f32(&self, pos_x: f32, pos_y: f32) -> f32 {
        let quad_x = pos_x.floor();
        let quad_y = pos_y.floor();
        let inner_x = pos_x - quad_x;
        let inner_y = pos_y - quad_y;

        self.corners_at_quad(quad_x as usize, quad_y as usize)
            .influence_at_f32(inner_x, inner_y)
    }
}

// [TODO] fractal noise
