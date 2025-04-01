//! Basic Perlin noise generation test

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

use loot_and_roam::common::terrain::noise::*;

#[test]
fn quad_influence_1() {
    let quad = LatticeQuadCorners::new(
        NoiseLatticePoint::new(-1.0, -1.0),
        NoiseLatticePoint::new(0.0, 0.0),
        NoiseLatticePoint::new(0.0, 0.0),
        NoiseLatticePoint::new(-1.0, -1.0),
    );

    assert!(quad.influence_at(0.1, 0.1) < 0.0);
    assert!(quad.influence_at(0.9, 0.9) > 0.0);
}

#[test]
fn quad_lookup() {
    let lattice = NoiseLattice::new(3, 3);
    let quad_1 = lattice.corners_at_quad(1, 0);
    let quad_2 = lattice.corners_at_quad(1, 1);

    assert_eq!(quad_1.sw, quad_2.nw);
    assert_eq!(quad_1.se, quad_2.ne);
}
