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

use bevy::prelude::*;
use std::time::Duration;

use derive_builder::Builder;
use rand::{thread_rng, Rng};

use crate::common::{
    prelude::{
        default_modulator, CenterPoint, FractalNoise, ModulationParams, TerrainGeneratorBuilder,
    },
    state::{GameState, SceneSetupEvent},
    terrain::buffer::TerrainBuffer,
};

/// Parameters used to construct a new overworld scene.
#[derive(Debug, Builder, Clone)]
pub struct OverworldSceneParams {
    /// Modulates the size of the island.
    ///
    /// Affects both the number of 'terrain seeds' (round areas around which
    /// to generate land), and the average size of each seed.
    ///
    /// 32 is the default, 255 is the maximum.
    pub island_size: u8,

    /// How well defended the island should be, inland.
    ///
    /// Controls the placement of defensive props.
    pub prop_defense: u8,

    /// How many patrol paths to generate.
    ///
    /// Whether they will be used depends on how many armed ships spawn.
    pub patrol_paths: u8,

    /// How many ships visit this island.
    ///
    /// The bigger this value, the faster the frequency with which new NPC
    /// ships are spawned in the island.
    ///
    /// 0 means no visits, 16 means a visit every minute.
    pub visit_frequency: u8,

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
    pub spawn_unarmed: u8,

    /// How many armed ships to spawn.
    pub spawn_armed: u8,

    /// How likely each spawned armed ship is to patrol the island.
    ///
    /// Chance calculation is (value + 1) / 256.
    ///
    /// 255 for always, 0 for a 1 in 256 chance.
    pub patrol_occupancy: u8,
}

impl Default for OverworldSceneParams {
    fn default() -> Self {
        Self {
            island_size: 32,
            prop_defense: 10,
            patrol_paths: 2,
            visit_frequency: 50,
            spawn_unarmed: 30,
            spawn_armed: 5,
            patrol_occupancy: 90,
        }
    }
}

impl OverworldSceneParams {
    /// Samples a random number of terrain seeds to generate, from the island
    /// size parameter.
    pub fn terrain_num_seeds<R: Rng + Sized>(&self, rng: &mut R) -> u8 {
        let choice: u8 = rng.random_range(3..16);

        ((self.island_size as u16 * choice as u16).isqrt() / 80) as u8
    }

    /// Samples a random terrain seed [CenterPoint] from the island size parameter.
    pub fn terrain_next_center_point<R: Rng + Sized>(&self, rng: &mut R) -> CenterPoint {
        let base_scale: f32 = 0.8 / 32.0;
        let size = self.island_size as f32 * base_scale;

        let mut coord = || rng.random_range(-size * 120.0..size * 120.0).sqrt();

        CenterPoint::new(Vec2::new(coord(), coord()), size)
    }

    /// How often a new ship would visit an overworld scene generated with
    /// these settings.
    pub fn visit_interval(&self) -> Option<Duration> {
        if self.visit_frequency == 0 {
            None
        } else {
            Some(Duration::from_secs_f32(
                60.0 * 16.0 / self.visit_frequency as f32,
            ))
        }
    }

    /// Computes the probability of a spawned armed ship occupying a patrol
    /// route, between 0 and 1, and returns as a f64 floating-point value.
    pub fn patrol_chance_f64(&self) -> f64 {
        (self.patrol_occupancy + 1) as f64 / 256.0
    }

    /// Computes the probability of a spawned armed ship occupying a patrol
    /// route, between 0 and 1, and returns as a f32 floating-point value.
    pub fn patrol_chance_f32(&self) -> f32 {
        (self.patrol_occupancy + 1) as f32 / 256.0
    }
}

/// This reosurce controls the creation of Overworld scenes.
///
/// It is set at the start of the game (in [GameEvent::Start]) when choosing an
/// initial island to raid, and in the intermission (in
/// [GameEvent::Intermission]) when picking an island in the Observatory.
#[derive(Resource, Default, Clone, Debug)]
pub struct OverworldSceneInitializer {
    pub params: OverworldSceneParams,
}

impl OverworldSceneInitializer {
    fn setup_overworld_island(
        &self,
        scene_tree: Entity,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        // [TODO] use a Bevy resource to store a common RNG
        let mut rng = rand::rng();

        let num_seeds = self.params.terrain_num_seeds(&mut rng);

        let center_points = vec![(); num_seeds as usize]
            .iter()
            .map(|_| self.params.terrain_next_center_point(&mut rng))
            .collect::<Vec<_>>();

        let terragen = TerrainGeneratorBuilder::default()
            .noise(FractalNoise::random_octaves(
                10.0,
                10.0,
                4.try_into().unwrap(),
                &mut rng,
            ))
            .modulator(default_modulator())
            .modulation_params(ModulationParams {
                min_shore_distance: 4.0,
                max_shore_distance: 14.0,
                ..Default::default()
            })
            .center_points(center_points)
            .resolution(10.0)
            .build()
            .unwrap();

        let terrain = TerrainBuffer::generate(terragen, 0.3, 3.0, 80.0);

        let terrain_entity = commands
            .spawn((
                terrain.as_bundle(meshes),
                MeshMaterial3d(materials.add(Color::srgb_u8(80, 190, 45))),
                Transform::from_xyz(0.0, -40.0, 0.0),
            ))
            .id();
        commands.entity(scene_tree).add_child(terrain_entity);
    }

    /// Initializes an overworld scene.
    pub(crate) fn setup_overworld(
        &self,
        scene_tree: Entity,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        self.setup_overworld_island(scene_tree, commands, meshes, materials);
    }
}

fn setup_overworld_scene(
    mut commands: Commands,
    mut setup_event: EventReader<SceneSetupEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    initializer: Res<OverworldSceneInitializer>,
) {
    for ev in setup_event.read() {
        initializer.setup_overworld(ev.scene_tree, &mut commands, &mut meshes, &mut materials);
    }
}

pub struct OverworldSceneSetupPlugin;

impl Plugin for OverworldSceneSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_overworld_scene.run_if(in_state(GameState::Overworld)),
        );
        app.init_resource::<OverworldSceneInitializer>();
    }
}
