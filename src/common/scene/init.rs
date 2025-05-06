//! # Scene initialization
//!
//! Defines a common procedure and parametrization for constructing "overworld
//! scenes".
//!
//! Those parameters can vary along a playthrough. For instance, picking
//! a different island at the end of the intermission can affect the size of
//! the island (and its cluster of 'terrain seeds'), as well as NPC instance
//! initialization and spawning behavior.

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

use std::time::Duration;

use bevy::math::Vec2;

use crate::common::prelude::{
    BaseModulationParams, BaseModulationParamsBuilder, CenterPoint, ModulationParams,
};

/// Parameters used to construct a new overworld scene.
pub struct OverworldParams {
    /// Modulates the size of the island.
    ///
    /// Affects both the number of 'terrain seeds' (round areas around which
    /// to generate land), and the average size of each seed.
    ///
    /// 32 is the default, 255 is the maximum.
    island_size: u8,

    /// How well defended the island should be, inland.
    ///
    /// Controls the placement of defensive props.
    prop_defense: u8,

    /// How many patrol paths to place.
    ///
    /// Whether they will be used depends on how many armed ships spawn.
    patrol_chance: u8,

    /// How many ships visit this island.
    ///
    /// The bigger this value, the faster the frequency with which new NPC
    /// ships are spawned in the island.
    ///
    /// 0 means no visits, 64 means a visit every minute.
    visit_frequency: u8,

    // [TODO] Split ship spawning parameters into ship classes, like in the prototype:
    // * fishers
    // * merchants
    // * travelers & joy-sailors
    // * mercenaries
    // * private escorts (possibly escorting smaller ships)
    // * army ships (small and large)
    // * expedition ships (bringing valuables from far away lands)
    // The following options for NPC ship spawning are very, VERY temporary.
    // ---
    /// How many unarmed ships to spawn.
    spawn_unarmed: u8,

    /// How many armed ships to spawn.
    spawn_armed: u8,

    /// How likely an armed ship is to be spawned patrolling the island.
    ///
    /// Chance calculation is (value + 1) / 256.
    ///
    /// 255 for always, 0 for a 1 in 256 chance.
    patrol_occupancy: u8,
}

impl OverworldParams {
    /// Samples a random number of terrain seeds to generate, from the island
    /// size parameter.
    pub fn terrain_num_seeds<R: Rng + Sized>(&self, &mut rng: R) -> u8 {
        let choice: u8 = rng.random_range(3..16);

        ((self.island_size as u16 * choice as u16).isqrt() / 80) as u8
    }

    /// Samples a random terrain seed size value from the island size parameter.
    pub fn terrain_next_seed_size<R: Rng + Sized>(&self, &mut rng: R) -> f32 {
        let base_scale: f32 = 0.8 / 32.0;
        let size_f32: f32 = self.island_size as f32 * base_scale;

        size_f32 * base_scale
    }

    /// Samples a random terrain seed [CenterPoint] parameter set from the island size
    /// parameter.
    pub fn terrain_next_seed_size<R: Rng + Sized>(&self, &mut rng: R) -> CenterPoint {
        let size = self.terrain_seed_next_size(rng);

        let coord = || rng.random_range(-size * 120..size * 120).sqrt();

        CenterPoint::new(Vec2::new(coord(), coord()), size)
    }

    /// How often a new ship would visit an overworld scene generated with
    /// these settings.
    pub fn visit_interval(&self) -> Option<Duration> {
        if self.visit_frequency == 0 {
            None
        } else {
            Some(Duration::from_secs_f32(60.0 / self.visit_frequency as f32))
        }
    }

    /// Computes the probability of a spawned armed ship occupying a patrol
    /// route, between 0 and 1, and returns as a f64 floating-point value.
    pub fn patrol_chance_f64(&self) -> f64 {
        (self.patrol_chance + 1) as f64 / 256.0
    }

    /// Computes the probability of a spawned armed ship occupying a patrol
    /// route, between 0 and 1, and returns as a f32 floating-point value.
    pub fn patrol_chance_f32(&self) -> f32 {
        (self.patrol_chance + 1) as f32 / 256.0
    }
}
