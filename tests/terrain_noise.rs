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
        NoiseLatticePoint::new(-127, -127),
        NoiseLatticePoint::new(0, 0),
        NoiseLatticePoint::new(0, 0),
        NoiseLatticePoint::new(-127, -127),
    );

    assert!(quad.influence_at_i8(0, 0) < 0);
    assert!(quad.influence_at_i8(127, 127) > 0);
}
