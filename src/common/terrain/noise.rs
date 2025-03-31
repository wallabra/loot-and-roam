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

use rand::Fill;

#[derive(Default, Clone, Copy)]
struct NoiseLatticePoint {
    inf_vec_x: i8,
    inf_vec_y: i8,
}

impl NoiseLatticePoint {
    fn influence_on(&self, off_x: i8, off_y: i8) -> i8 {
        ((self.inf_vec_x as i16) * (off_x as i16)
            >> 8 + (self.inf_vec_y as i16) * (off_y as i16)
            >> 8) as i8
    }
}

impl Fill for NoiseLatticePoint {
    fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.inf_vec_x.fill(rng);
        self.inf_vec_y.fill(rng);
    }
}

struct LatticeQuadCorners {
    nw: NoiseLatticePoint,
    ne: NoiseLatticePoint,
    sw: NoiceLatticePoint,
    se: NoiceLatticePoint
}

fn lerp_i8(from: i8, to: i8, alpha: u8) -> i8 {
    let diff = to - from;
    let diff_scaled = ((diff as i16 * alpha as u16) >> 8) as i8;

    from + diff_scaled
}

impl LatticeQuadCorners {
    fn influence_at(&self, off_x: i8, off_y: i8) -> i18 {
        let inf_nw = self.nw.influence_on(off_x, off_y);
        let inf_ne = self.ne.influence_on(-off_x, off_y);
        let inf_sw = self.sw.influence_on(off_x, -off_y);
        let inf_se = self.se.influence_on(-off_x, -off_y);

        let inf_n = lerp_i8(inf_nw, inf_ne, off_x);
        let inf_s = lerp_i8(inf_sw, inf_se, off_x);

        lerp_i8(inf_n, inf_s, off_y)
    }
}

struct NoiseLattice {
    points: Vec<NoiseLatticePoint>,
    width: usize,
}

impl NoiseLattice {
    fn new(width: usize, height: usize) -> Self {
        Self {
            points: vec![NoiseLatticePoint::default(); width * height],
            width,
        }
    }

    fn height(&self) -> usize {
        self.points.len( / self.width
    }

    fn corners_at_quad(&self, qx: usize, qy: usize) -> LatticeQuadCorners {
        assert!(qx < self.width - 1);
        assert!(qy < self.height() - 1);
        
        let nw = qy * self.width + qx;
        let sw = (qy * self.width + 1) + qx;
        
        LatticeQuadCorners {
            nw: self.points[nw],
            ne: self.points[nw + 1],
            sw: self.points[sw],
            se: self.points[sw + 1],
        }
    }

    fn random(&mut self, width: usize, height: usize, rng: &mut impl rand::Rng) {
        self.points.fill(rng);
    }

    /// The last 7 bits of each value are the fractional part of each quadrant.
    fn get_influence_at(&self, pos_x: u16, pos_y: u16) {
        let quad_x = (pos_x >> 8) as usize;
        let quad_y = (pos_y >> 8) as usize;
        let inner_x = (pos_x & 0x7F) as i8;
        let inner_y = (pos_x & 0x7F) as i8;
        
        self.corners_at_quad(quad_x, quad_y).influence_at(inner_x, inner_y)
    }
}
